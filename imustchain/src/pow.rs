use std::time::Duration;
use std::thread;
use sha2::{Sha256, Digest};

use crate::Transaction;

pub struct ProofOfWork {
    pub difficulty: usize,
}

impl ProofOfWork {
    pub fn new(difficulty: usize) -> Self {
        Self { difficulty }
    }

    pub fn mine(&self, data: &Vec<Transaction>, index: u32, previous_hash: &str) -> (String, u64) {
        let mut nonce = 0;
        let target_prefix = "0".repeat(self.difficulty);

        loop {
            
            let input = format!("{}:{}:{:#?}:{}", index, previous_hash, data, nonce);

          
            let mut hasher = Sha256::new();
            hasher.update(input);
            let hash = format!("{:x}", hasher.finalize());

           
            if hash.starts_with(&target_prefix) {
                println!("⛏️ Block mined! Nonce: {}, Hash: {}", nonce, hash);
                thread::sleep(Duration::from_millis(3000));
                return (hash, nonce);
            }

            nonce += 1;
        }
    }
}
