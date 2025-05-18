use crate::block::Block;
use crate::user::UsersList;
use crate::consensus::pow::PoW;
use crate::consensus::poa::PoA;
use crate::blockchain::Blockchain;

pub trait Consensus {
    fn validate_block(&self,block:&mut Block,user_list:&mut UsersList) -> bool;
    fn mine_block(&mut self, block:&mut Block,blockchain:&Blockchain)-> bool;
}
pub enum HybridConsensus {
    PoW(PoW),
    PoA(PoA)
}
impl Consensus for HybridConsensus {
    fn validate_block(&self, block:&mut Block, user_list:& mut UsersList) -> bool {
        let tx_total_amount=block.total_transaction();
        match self {
            HybridConsensus::PoW(pow) if tx_total_amount> 1000 => pow.validate_block(block,user_list),
            HybridConsensus::PoA(poa) if tx_total_amount<=1000 => poa.validate_block(block, user_list),
            _=>false
        }
    }
    fn mine_block(&mut self,block:&mut Block,blockchain:&Blockchain) ->bool {
        let tx_count=block.total_transaction();
        match self {
            HybridConsensus::PoW(pow) if tx_count>1000 => pow.mine_block(block,blockchain),
            HybridConsensus::PoA(poa) if tx_count<=1000 => poa.mine_block(block,blockchain),
            _=> false
        }
    }
}


