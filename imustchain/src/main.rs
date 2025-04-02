use imustchain::Block;
use imustchain::Blockchain;
use imustchain::Transaction;


// Main function for the blockchain simulation
fn main() {
    println!("\n⛏️  Let's start mining and simulating transactions!\n");
    let traders = vec!["Alice", "Bob", "John", "Obama", "Biden", "Trump", "Bush", "Teddy"];
    let miner = traders[0].to_string();
   
    let mut testchain = Blockchain::new();
   
    let mut sender = miner.clone();
    let mut transaction = Vec::new(); 
    println!("🧱 Mining block...⛏️");
        for i in 0..traders.len() {
        let recipient = if i < traders.len() - 1 {
            traders[i + 1].to_string()
        } else {
            miner.clone() 
        };
        println!("Tx :{}, {} send to {}", i+1, sender, recipient);
        let mut tx = Transaction::new((i+1) as u64,format!("{} sent to {}", sender, recipient));
        tx.tx_hash=tx.hash_cal();
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