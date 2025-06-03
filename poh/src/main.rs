use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Validator {
    id: String,
    stake: u64,
}

#[derive(Debug, Clone)]
struct Block {
    previous_hash: String,
    transactions: Vec<String>,
    sequence_number: u64,
    timestamp: u64,
}

struct PoHSystem {
    validators: Vec<Validator>,
    hash_sequence: Vec<(String, u64)>,
    current_hash: String,
}

impl PoHSystem {
    fn new(validators: Vec<Validator>) -> PoHSystem {
        let init_hash = format!("{:x}", Sha256::digest(b"genesis"));
        PoHSystem { validators, hash_sequence: vec![(init_hash.clone(), 0)], current_hash: init_hash }
    }

    fn select_leader(&self) -> &Validator {
        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut target = seed % total_stake;
        for validator in &self.validators {
            if target < validator.stake {
                return validator;
            }
            target -= validator.stake;
        }
        self.validators.last().unwrap()
    }

    fn generate_hash_sequence(&mut self, transactions: Vec<String>, count: u64) {
        let mut last_hash = self.current_hash.clone();
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        for i in 0..count {
            let input = format!("{}{}{}", last_hash, start_time + i, transactions.get(0).unwrap_or(&String::new()));
            let hash = format!("{:x}", Sha256::digest(input.as_bytes()));
            self.hash_sequence.push((hash.clone(), start_time + i));
            last_hash = hash;
        }
        self.current_hash = last_hash;
    }

    fn create_block(&mut self, transactions: Vec<String>, previous_hash: &str) -> Block {
        let leader_id = self.select_leader().id.clone(); // Store ID to drop borrow
        self.generate_hash_sequence(transactions.clone(), 1);
        let sequence_number = self.hash_sequence.len() as u64;
        println!("领导者 {} 生成区块, 序列号: {}", leader_id, sequence_number);
        Block {
            previous_hash: previous_hash.to_string(),
            transactions,
            sequence_number,
            timestamp: self.hash_sequence.last().unwrap().1,
        }
    }

    fn verify_block(&self, block: &Block) -> bool {
        let last_seq = self.hash_sequence.get(block.sequence_number as usize - 1);
        last_seq.is_some() && last_seq.unwrap().1 == block.timestamp
    }
}

fn main() {
    let validators = vec![
        Validator { id: "V1".to_string(), stake: 100 },
        Validator { id: "V2".to_string(), stake: 200 },
        Validator { id: "V3".to_string(), stake: 300 },
    ];
    let mut poh = PoHSystem::new(validators);
    let transactions = vec!["Alice 转账 10 到 Bob".to_string()];
    let block = poh.create_block(transactions, "0000abcdef");
    if poh.verify_block(&block) {
        println!("区块验证通过！序列号: {}, 时间戳: {}", block.sequence_number, block.timestamp);
    } else {
        println!("区块验证失败！");
    }
}