use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Node {
    id: String,
    is_primary: bool,
}

#[derive(Debug, Clone)]
struct Block {
    previous_hash: String,
    transactions: Vec<String>,
    timestamp: u64,
    sequence_number: u64,
}

#[derive(Debug)]
struct PbftSystem {
    nodes: Vec<Node>,
    current_view: u64,
    f: usize,
}

impl PbftSystem {
    fn new(node_count: usize, f: usize) -> PbftSystem {
        let nodes = (0..node_count)
            .map(|i| Node {
                id: format!("Node{}", i),
                is_primary: i == 0,
            })
            .collect();
        PbftSystem { nodes, current_view: 0, f }
    }

    fn get_primary(&self) -> &Node {
        self.nodes.iter().find(|n| n.is_primary).unwrap()
    }

    fn process_request(&mut self, transactions: Vec<String>, previous_hash: &str, sequence_number: u64, is_faulty: bool) -> Option<Block> {
        if is_faulty {
            self.view_change();
            if self.get_primary().id == "Node0" { // 模拟新主节点非故障
                return None;
            }
        }
        if self.nodes.len() >= 2 * self.f + 1 {
            let block = Block {
                previous_hash: previous_hash.to_string(),
                transactions,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                sequence_number,
            };
            println!("生成区块: Seq {}, 视图: {}", sequence_number, self.current_view);
            return Some(block);
        }
        None
    }

    fn verify_block(&self, block: &Block) -> bool {
        block.sequence_number == self.current_view + 1
    }

    fn view_change(&mut self) {
        self.current_view += 1;
        let new_primary_index = (self.current_view as usize) % self.nodes.len();
        for node in &mut self.nodes {
            node.is_primary = false;
        }
        self.nodes[new_primary_index].is_primary = true;
        println!("视图变更！新主节点: {}", self.nodes[new_primary_index].id);
    }
}

fn main() {
    let mut pbft = PbftSystem::new(4, 1);
    let transactions = vec![
        String::from("Alice 转账 10 到 Bob"),
        String::from("Bob 转账 5 到 Charlie"),
    ];
    let previous_hash = "0000abcdef1234567890";

    // 第一次请求
    if let Some(block) = pbft.process_request(transactions.clone(), previous_hash, 1, false) {
        if pbft.verify_block(&block) {
            println!("区块验证通过！序列号: {}", block.sequence_number);
        }
    }

    // 模拟主节点故障并重试
    println!("\n模拟主节点故障...");
    if let Some(block) = pbft.process_request(
        vec![String::from("Charlie 转账 20 到 Alice")],
        previous_hash,
        2,
        true,
    ) {
        if pbft.verify_block(&block) {
            println!("区块验证通过！序列号: {}", block.sequence_number);
        }
    } else {
        println!("共识失败，视图已变更");
    }
}