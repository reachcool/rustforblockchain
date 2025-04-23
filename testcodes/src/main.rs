mod block;
mod transaction;
mod blockchain;
use block::Block;
use transaction::Transaction;
use blockchain::Blockchain;
fn main() {
    let traders = vec!["Alice", "Bob", "John", "Obama", "Biden", "Trump", "Bush"];
    let miner = traders[0].to_string();
    let mut testchain = Blockchain::new();
    let mut transaction = Vec::new(); 
    println!("\n⛏️  Let's start mining and simulating transactions!\n");
    let mut sender = miner.clone();
    for i in 0..traders.len() {
        println!("🧱 Mining block {}...⛏️", i + 1);
        let recipient = if i < traders.len() - 1 {
            traders[i + 1].to_string()
        } else {
            miner.clone() 
        };
        let trans_data = format!("{} sent to {}", sender, recipient);
        let mut tx = Transaction::new((i + 1) as u64,String::from(trans_data));
        tx.tx_hash=tx.hash_cal();
        println!("✉️ Transaction: {:#?}", tx); 
      
        sender = recipient; 
        println!();
        transaction.push(tx);
    } 
    let new_block = Block::new(1, String::new(), transaction);
    testchain.add_block(new_block);
    let total_blocks = testchain.chain.len();
    println!("✅ Total blocks added to the blockchain: {}", total_blocks);
    println!("the chain Details: {:#?}", testchain);
}