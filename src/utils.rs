use crate::block::Transaction;
use crate::block::{Block,User};
use crate::consensus;
use aes_gcm::aead::Aead;
use aes_gcm::AeadCore;
use aes_gcm::Key;
use chrono::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use ed25519_dalek::Verifier;
use generic_array::typenum::U12;
use sha2::{Sha256,Digest};
use hex;
use ed25519_dalek;
use argon2::{password_hash::{
    rand_core::OsRng,
    SaltString
},Argon2};
use aes_gcm::Nonce;
use aes_gcm::{KeyInit,aead::generic_array::GenericArray};
use aes_gcm::Aes256Gcm;
use std::error::Error;
use core::convert::TryInto;


impl Block {
    pub fn total_transaction (&self)-> u64 {
        self.data.iter().map(|tx|tx.amount).sum()
    }
    pub fn new(index:u32,prev_hash:String,data:Vec<Transaction>,hash:String,nonce:u64,validator:String) -> Self {
        let timestamp = Utc::now().timestamp_millis();
        let temp_block = Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
            nonce,
            validator
        };
        temp_block
    }
}
impl User {
   pub fn new (uname:&str,pwd:&str,initial_balance:u64,validator_status:bool) ->Self {
    let (pubkey,encrypted_private_key,nonce,salt,password_hashed) =generate_secure_keypair(uname, pwd);
    let temp_user = User {
        username:uname.to_string(),
        password: password_hashed,
        public_key:hex::encode(pubkey.to_bytes()),
        encrypted_private_key:hex::encode(&encrypted_private_key),
        salt: salt.to_string(),
        nonce:hex::encode(&nonce),
        balance:initial_balance,
        is_validator:validator_status
    };
    let mut poa= consensus::poa::PoA {validator_list:Vec::new()};
    if temp_user.is_validator == true {poa.validator_list.push(temp_user.public_key.clone())}
    temp_user
   } 
   pub fn decrypt_private_key(&self,password:&str)->Result<SigningKey, Box<dyn std::error::Error>> {
    let input=format!("{},{}",self.username,password);
    let mut seed=[0u8;32];
    let argon2=Argon2::default();
    let salt_bytes=self.salt.as_bytes();
    argon2.hash_password_into(input.as_bytes(), salt_bytes, &mut seed)
        .expect("failed to derive key");
    let aes_key=aes_gcm::Key::<Aes256Gcm>::from_slice(&seed);
    let chiper=Aes256Gcm::new(aes_key);
    let nonce_bytes=hex::decode(&self.nonce).expect("invalid nonce");
    let nonce=Nonce::from_slice(&nonce_bytes);
    let encrypted_key_bytes=hex::decode(&self.encrypted_private_key).expect("invalid encrypted key");
    let decrypted_bytes=chiper.decrypt(nonce, encrypted_key_bytes.as_ref())
        .expect("decryption failed");
    let key_bytes:[u8;32]=decrypted_bytes
        .try_into()
        .map_err(|_|"invalid key length")?;
    let signing_key=SigningKey::from_bytes(&key_bytes);
    Ok(signing_key)
   }
   pub fn sign_transaction(&self,tx: &mut Transaction,password:&str)-> Result<(),Box<dyn Error>> {
    let key=self.decrypt_private_key(password)?;
    tx.sign(&key);
    Ok(())
   }

}
pub fn generate_secure_keypair(username:&str, password:&str)->(VerifyingKey,Vec<u8>,GenericArray<u8,U12>,SaltString,String) {
        let input = format!("{},{}",username,password);
        let salt=SaltString::generate(&mut OsRng);
        let mut seed=[0u8;32];
        Argon2::default()
            .hash_password_into(input.as_bytes(), salt.as_str().as_bytes(), &mut seed)
            .expect("failed to hash");
        let password_hashed= hex::encode(&seed);
        let secret = SigningKey::from_bytes(&seed);
        let pubkey= secret.verifying_key(); 
        // hash the secret key using aesgcm
        let aes_key = Key::<Aes256Gcm>::from_slice(&seed);
        let chipper=Aes256Gcm::new(aes_key);
        let nonce=Aes256Gcm::generate_nonce(&mut OsRng);
        let chipper_text=chipper.encrypt(&nonce, secret.to_bytes().as_ref()).expect("failed to enrcypt");
        (pubkey,chipper_text,nonce,salt,password_hashed)
    }

impl Transaction {
    pub fn sign(& mut self,signing_key:&SigningKey)->String {
       // hash transaction data
       let mut hasher = Sha256::new();
       hasher.update(self.message.as_bytes());
       let hashed_message= hasher.finalize();
       //Sign the hash
       let signature=signing_key.sign(&hashed_message);
       //store signature as hex sting
       let signature_hex=hex::encode(signature.to_bytes());
       self.signature=signature_hex.clone();
       signature_hex
    }
    pub fn verify_sign(&mut self)->bool {
        //decode the pubkey
       let sender_bytes=hex::decode(&self.sender).expect("failed to decode");
       let sender_array:[u8;32] = sender_bytes.try_into().unwrap();
       let sender_pubkey=ed25519_dalek::VerifyingKey::from_bytes(&sender_array).expect("failed to get verifying key");
       // get the signature
       let signature_bytes=hex::decode(&self.signature).expect("failed to get signature");
       let signature_array:[u8;64] = signature_bytes.try_into().unwrap();
       let signature_format = Signature::from_bytes(&signature_array);
       // get message to bytes
       let message_bytes = self.message.as_bytes();
       //hash the message
       let mut hasher=Sha256::new();
       hasher.update(message_bytes);
       let hashed_message=hasher.finalize();
       // verify the transaction
       sender_pubkey.verify(&hashed_message, &signature_format).is_ok()
    }
    pub fn verify_balance(&self,user:&User)->bool {
        let user_balance=user.balance;
        // verify the amount of transaction
        if user_balance < self.amount {
            println!("insufficient account balance");
            return false
        } else {
            return true
        }
    }
    pub fn new (sender_pubkey:&str,receiver_pubkey:&str,transaction_amount:u64)->Self {
        let temp_transaction = Transaction {
            sender: sender_pubkey.to_string(),
            receiver: receiver_pubkey.to_string(),
            amount:transaction_amount,
            signature: String::new(),
            message: format!("{},{},{}", sender_pubkey,receiver_pubkey,transaction_amount)
        };
        temp_transaction
    }
}
