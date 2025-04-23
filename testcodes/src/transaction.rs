use sha2::{Digest, Sha256}; 
use std::time::{SystemTime, UNIX_EPOCH}; 

#[derive(Debug)]
pub struct Transaction {
    tx_id: u64,
    timestamp: u64,
    tx_data: String,
    pub tx_hash: String
}
impl  Transaction {

    pub fn new(tx_id:u64,tx_data: String)->Transaction{
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Transaction {
            tx_id,
            timestamp,
            tx_data,
            tx_hash: String::new()
        }
    }
    pub fn hash_cal(&mut self) -> String {
      
        let data = format!(
            "{}{}{}",    
            self.tx_id,
            self.timestamp,
            &self.tx_data
        ); 
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);
        hash_str
    }
}

