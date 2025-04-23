//block.rs
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};  
use std::fmt;
use chrono::DateTime;
use chrono::Utc;
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
    merkle_root:String                
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
            merkle_root:String::new(),   
        }
    }  
    
    /// 计算 Merkle Root：动态处理任意交易数的 Merkle Tree
    fn calculate_merkle_root(&self) -> String {
        let mut hashes: Vec<String> = self
            .trans_data
            .iter()
            .map(|t| t.tx_hash.clone())
            .collect();

        if hashes.is_empty() {
            return String::new();
        }
        if hashes.len() == 1 {
            return hashes[0].clone();
        }

        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0]) // 奇数时复制最后一个
                };
                let mut hasher = Sha256::new();
                hasher.update(combined.as_bytes());
                let hash = format!("{:x}", hasher.finalize());
                next_level.push(hash);
            }
            hashes = next_level;
        }

        hashes[0].clone()
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
            self.merkle_root = self.calculate_merkle_root();
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
    /// 格式化 Block 输出，展示区块信息。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime: DateTime<Utc> = DateTime::from_timestamp(self.timestamp as i64, 0)
            .unwrap_or(DateTime::from_timestamp(0, 0).unwrap());
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        write!(
            f,
            "Block #{}:\n  Data: {:#?}\n  Timestamp: {}\n  Previous Block Hash: {}\n Block Hash: {}\n  Nonce: {}, MerkleRoot Hash :{}",
            self.index,
            self.trans_data,
            formatted_time,
            self.previous_hash,
            self.hash,
            self.nonce,
            self.merkle_root,
        )
    }
}