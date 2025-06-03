use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{SystemTime, UNIX_EPOCH};

// 定义验证者结构体
#[derive(Debug, Clone)]
struct Validator {
    id: String,
    stake: u64, // 质押量
}

// 定义区块结构体
#[derive(Debug, Clone)]
struct Block {
    previous_hash: String,
    transactions: Vec<String>,
    timestamp: u64,
    validator_id: String, // 生成区块的验证者 ID
}

impl Block {
    // 创建新区块
    fn new(previous_hash: &str, transactions: Vec<String>, validator_id: &str) -> Block {
        Block {
            previous_hash: previous_hash.to_string(),
            transactions,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("时间获取失败")
                .as_secs(),
            validator_id: validator_id.to_string(),
        }
    }
}

// PoS 系统结构体
#[derive(Debug)]
struct PosSystem {
    validators: Vec<Validator>,
}

impl PosSystem {
    // 创建 PoS 系统
    fn new(validators: Vec<Validator>) -> PosSystem {
        PosSystem { validators }
    }

    // 加权随机选择验证者
    fn select_validator(&self) -> &Validator {
        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();
        if total_stake == 0 {
            panic!("总质押量为 0，无法选择验证者");
        }

        // 使用时间戳作为随机种子
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("时间获取失败")
            .as_secs();
        let mut rng = StdRng::seed_from_u64(seed);

        // 加权随机选择
        let mut target = rng.gen_range(0..total_stake);
        for validator in &self.validators {
            if target < validator.stake {
                return validator;
            }
            target -= validator.stake;
        }
        // 防止意外情况，返回最后一个验证者
        self.validators.last().expect("验证者列表为空")
    }

    // 生成新区块
    fn create_block(&self, previous_hash: &str, transactions: Vec<String>) -> Block {
        let validator = self.select_validator();
        println!("选中验证者: {}, 质押量: {}", validator.id, validator.stake);
        Block::new(previous_hash, transactions, &validator.id)
    }

    // 验证区块
    fn verify_block(&self, block: &Block) -> bool {
        // 检查验证者是否在列表中且质押量有效
        self.validators
            .iter()
            .any(|v| v.id == block.validator_id && v.stake > 0)
    }
}

fn main() {
    // 初始化验证者
    let validators = vec![
        Validator {
            id: String::from("Validator1"),
            stake: 100,
        },
        Validator {
            id: String::from("Validator2"),
            stake: 200,
        },
        Validator {
            id: String::from("Validator3"),
            stake: 300,
        },
    ];

    // 创建 PoS 系统
    let pos_system = PosSystem::new(validators);

    // 创建测试区块
    let transactions = vec![
        String::from("Alice 转账 10 到 Bob"),
        String::from("Bob 转账 5 到 Charlie"),
    ];
    let block = pos_system.create_block("0000abcdef1234567890", transactions);

    // 验证区块
    if pos_system.verify_block(&block) {
        println!("区块验证通过！,区块信息：{:#?}", block);
    } else {
        println!("区块验证失败！");
    }
}