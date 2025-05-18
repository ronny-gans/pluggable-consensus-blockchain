use crate::{block::Block, blockchain::Blockchain};
use crate::user::UsersList;
use rand::{rng,Rng};
use sha2::{Sha256,Digest};
use hex;
use std::clone::Clone;

#[derive(Clone)]
pub struct PoA {
    pub validator_list: Vec<String>
}
impl PoA {
    pub fn new() -> PoA {
        PoA { validator_list: Vec::new() }
    }
    pub fn mine_block (&mut self,block:&mut Block,_blockchain:&Blockchain)->bool {
        if self.validator_list.is_empty() {
            eprintln!("Validator list is empty. Cannot mine block.");
            return false;
        }
        // get random validator
        let mut rng=rng();
        let random_index=rng.random_range(0..self.validator_list.len());
        let validator = self.validator_list[random_index].clone();
        // generate hash transaction
        let header=format!("{},{},{:?},{},{}",block.index,block.timestamp,block.data,block.prev_hash,block.nonce);
        let mut hasher=Sha256::new();
        hasher.update(header);
        let hash_bytes=hasher.finalize();
        let hash=hex::encode(hash_bytes);
        block.hash=hash;
        block.validator=validator;
        println!("mining by {}, with hash{}",block.validator,block.hash);
        return true
    }
    pub fn validate_block(&self,block:&mut Block, user_list:&mut UsersList)->bool {
        let header=format!("{},{},{:?},{},{}",block.index,block.timestamp,block.data,block.prev_hash,block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(header);
        let hash_bytes=hasher.finalize();
        let expected_hash=hex::encode(hash_bytes);
        let hash_matches=block.hash==expected_hash;
        let validator_valid=self.validator_list.contains(&block.validator);
        if hash_matches&&validator_valid {
            block.nonce=0x0000000000000000;
            for transaction in block.data.iter() {
                if let Some(sender) = user_list.users.iter_mut().find(|u| u.public_key == transaction.sender) {
                    sender.balance-=transaction.amount;
                }
                if let Some(receiver) = user_list.users.iter_mut().find(|u| u.public_key == transaction.receiver) {
                    receiver.balance+=transaction.amount;
                }
            }
            return true
        } else {
            block.nonce=0x0000000000000001;
            eprintln!(
                "Block validation failed. Hash match: {}, Validator valid: {}",
                hash_matches, validator_valid);
            return false
        }
    }
}