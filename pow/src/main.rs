use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

// 定义区块结构体
#[derive(Debug, Clone)]
struct Block {
    previous_hash: String,
    transactions: Vec<String>,
    timestamp: u64,
    nonce: u64,
    difficulty: usize,
}

impl Block {
    // 创建新区块
    fn new(previous_hash: &str, transactions: Vec<String>, difficulty: usize) -> Block {
        Block {
            previous_hash: previous_hash.to_string(),
            transactions,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            nonce: 0,
            difficulty,
        }
    }

    // 计算区块的哈希值
    fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}",
            self.previous_hash,
            self.transactions.join(""),
            self.timestamp,
            self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 检查哈希值是否满足难度条件
    fn is_valid_hash(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.difficulty))
    }

    // 挖矿函数
    fn mine_block(&mut self) -> String {
        loop {
            let hash = self.calculate_hash();
            if self.is_valid_hash(&hash) {
                println!("挖矿成功！哈希值: {}, Nonce: {}, txs {:#?}", hash, self.nonce,self.transactions);
                return hash;
            }
            self.nonce += 1;
        }
    }

    // 验证区块
    fn verify_block(&self, hash: &str) -> bool {
        self.calculate_hash() == hash && self.is_valid_hash(hash)
    }
}

fn main() {
    // 创建测试区块
    let mut block = Block::new(
        "0000abcdef1234567890",
        vec![
            String::from("Alice 转账 10 到 Bob"),
            String::from("Bob 转账 5 到 Charlie"),
        ],
        4, // 难度：哈希值前4位为0
    );

    // 执行挖矿
    let hash = block.mine_block();

    // 验证区块
    if block.verify_block(&hash) {
        println!("区块验证通过！");
    } else {
        println!("区块验证失败！");
    }
}