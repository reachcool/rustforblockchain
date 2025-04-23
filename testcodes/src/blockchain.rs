// blockchain.rs
use crate::block::Block;
use crate::transaction::Transaction;


#[derive(Debug)]
pub struct Blockchain {
   pub  chain: Vec<Block>, 
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut genes_tx = Transaction::new(1,String::from("This is an Genesis Tx"));
        genes_tx.tx_hash = genes_tx.hash_cal();
        let mut genesis_block = Block::new(0, String::new(), vec![genes_tx]);
        genesis_block.mine();
        
        Blockchain {
            chain: vec![genesis_block], 
        }
    }
 
   pub fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone(); 
        new_block.previous_hash = previous_hash;
        new_block.mine();
        
        self.chain.push(new_block); 
    }
}