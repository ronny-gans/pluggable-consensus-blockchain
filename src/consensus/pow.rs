
use crate::{block::Block, user::UsersList};
use sha2::{Sha256,Digest};
use crate::blockchain::Blockchain;
use std::clone::Clone;

#[derive(Clone)]
pub struct PoW {
    pub difficulty: u64,
}

impl PoW {
    pub fn mine_block(&mut self,block:&mut Block,blockchain:&Blockchain) ->bool {
        //adjust difficulty
        if let Some(last_block) = blockchain.blocks.last() {
            let expected_time=60_000;
            let actual_time = block.timestamp-last_block.timestamp;
            if actual_time < expected_time {
                 self.difficulty+=1
            } else if actual_time>expected_time {
                self.difficulty=self.difficulty.saturating_sub(1)
            }
        }
        // get target hash
        let target_prefix = "0".repeat(self.difficulty as usize);
        block.nonce=0;
        //loop till get hash and nonce
        loop {
            let header=format!("{},{},{:?},{},{}",block.index,block.timestamp,block.data,block.prev_hash,block.nonce);
            let mut hasher = Sha256::new();
            hasher.update(header.as_bytes());
            let result=hasher.finalize();
            let hash=hex::encode(&result);

            if hash.starts_with(&target_prefix) {
                block.hash=hash;
                return true
            } else {
                block.nonce+=1
            }
        }
    }
    pub fn validate_block(&self, block:&Block, user_list:&mut UsersList)->bool {
        // get target hash
        let header=format!("{},{},{:?},{},{}",block.index,block.timestamp,block.data,block.prev_hash,block.nonce);
        let mut hasher = Sha256::new();
        hasher.update(header.as_bytes());
        let result=hasher.finalize();
        let hash=hex::encode(&result);
        let target_prefix="0".repeat(self.difficulty as usize);
        let validation_result=hash.starts_with(&target_prefix)&&hash==block.hash; 
        if validation_result == true {
            for transaction in block.data.iter() {
                if let Some(sender) = user_list.users.iter_mut().find(|u| u.public_key == transaction.sender) {
                    sender.balance-=transaction.amount;
                }
                if let Some(receiver) = user_list.users.iter_mut().find(|u| u.public_key == transaction.receiver) {
                    receiver.balance+=transaction.amount;
                    
                }
            } true  
        } else {
            false
        }
    }
}