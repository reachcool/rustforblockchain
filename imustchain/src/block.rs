//block.rs
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};  
use std::fmt;
use chrono::DateTime;
use std::thread; 
use std::time::Duration; 
use crate::Transaction;
#[derive(Debug)]
pub struct Block {
    index: u32,                     
    pub previous_hash: String,          
    timestamp: u64,                
    trans_data: Vec<Transaction>,             
    nonce: u64,                     
    pub hash: String,                   
}

impl Block {

    pub fn new(index: u32, previous_hash: String, trans_data: Vec<Transaction>) -> Block {
        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        Block {
            index,                    
            previous_hash,          
            timestamp,                
            trans_data,                    
            nonce: 0,                  
            hash: String::new(),      
        }
    }   
    pub fn hash_cal(&mut self) -> String {
       
        let data = format!(
            "{}{}{}{:#?}{}",
            self.index,
            &self.previous_hash,
            self.timestamp,
            &self.trans_data,
            self.nonce
        );
       
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

       
        let hash_str = format!("{:x}", result);
        hash_str
    }
    pub fn mine(&mut self) {
       
        loop {
            self.hash = self.hash_cal(); 
            if !self.hash.is_empty() && &self.hash[..2] == "00" {
               
                println!("⛏️ Block mined: {}", self.index);
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break; 
            }
           
            self.nonce += 1;
        }
        println!("hash: {}, nonce: {}", self.hash, self.nonce);
    }
}


impl fmt::Display for Block {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  
        let datetime = DateTime::from_timestamp(self.timestamp as i64, 0);
        write!(
            f,
            "Block {}: {:#?} at {:#?}, \n hashed as: {}",
            self.index, self.trans_data, datetime, self.hash
        )
    }
}
