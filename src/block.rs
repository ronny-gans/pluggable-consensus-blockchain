use serde::{Serialize,Deserialize};

# [derive(Serialize,Deserialize,Debug,Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub public_key: String,
    pub encrypted_private_key: String,
    pub salt:String,
    pub nonce:String,
    pub balance:u64,
    pub is_validator:bool
}

#[derive(Serialize,Deserialize,Debug,Clone)]
// declare the transaction
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: String,
    pub message: String
}

// declare block
# [derive(Serialize,Deserialize,Debug,Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub data: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub validator:String
}



