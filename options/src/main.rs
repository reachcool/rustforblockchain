// 定义区块链交易结构体
#[derive(Debug)]
struct Transaction {
    id: u64,
    amount: Option<f64>, // 交易金额，可能存在或缺失
}

// 处理交易金额，增加手续费
fn add_fee(amount: Option<f64>) -> Option<f64> {
    match amount {
        Some(value) => Some(value + 0.1), // 存在金额，加 0.1 手续费
        None => None,                    // 无金额，返回 None
    }
}

fn main() {
    // 示例 1：交易有金额
    let tx1 = Transaction {
        id: 1,
        amount: Some(10.0),
    };
    // 示例 2：交易无金额
    let tx2 = Transaction {
        id: 2,
        amount: None,
    };

    let result1 = add_fee(tx1.amount);
    let result2 = add_fee(tx2.amount);

    println!("交易 {} 金额调整后: {:?}", tx1.id, result1); // 输出: Some(10.1)
    println!("交易 {} 金额调整后: {:?}", tx2.id, result2); // 输出: None
}