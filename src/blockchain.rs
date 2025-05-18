use crate::block::{Block,Transaction};
use crate::consensus::module::Consensus;
use crate::user::UsersList;
use serde::{Serialize,Deserialize};
use serde_json;
use std::io::{self,Write};
use std::fs::File;

#[derive(Serialize,Deserialize)]
pub struct Blockchain {
    pub blocks:Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(
            0,
            "0".to_string(),
            Vec::new(),
            "genesis".to_string(),
            0,
            String::new()
        );
        Blockchain {blocks:vec![genesis_block]}
    }
    pub fn add_block(&mut self, data:Vec<Transaction>,consensus:&mut dyn Consensus,user_list:&mut UsersList) {
        let index= self.blocks.len() as u32;
        let prev_hash=if let Some(last)=self.blocks.last() {
            last.hash.clone()
        } else {
            String::from("0")
        };
        let hash=String::new();
        let nonce:u64=0;
        let validator:String=String::new();
        let mut new_block =Block::new(index,prev_hash,data,hash,nonce,validator);
        if consensus.mine_block(&mut new_block,&self) && consensus.validate_block(&mut new_block,user_list) {
            self.blocks.push(new_block);
            println!("blocks is successfully added")
        } else {
            eprintln!("failed to add block")
        }

    }
    pub fn get_all_blocks(&self) {
        for block in &self.blocks {
            println!(
                "{},{},{:?},{},{},{},{}",
                block.index,
                block.timestamp,
                block.data,
                block.prev_hash,
                block.hash,
                block.nonce,
                block.validator
            )
        }
    }
    pub fn save_to_file(&self,filename:&str)->io::Result<()> {
        let json=serde_json::to_string_pretty(&self.blocks).unwrap();
        let mut file=File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())

    }
}