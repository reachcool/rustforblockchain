use std::time::UNIX_EPOCH;
use std::time::SystemTime;

#[derive(Debug)]
enum TxStatus {
    Pending(u64, f64),                    // 提交时间, 预估费用
    Confirmed(u64, u64, String),         // 区块高度, 确认时间, 矿工地址
    Failed(String, u32)           // 原因，错误码
}

    
#[derive(Debug)]
struct Transaction {
    txhash: String,             
    amount: f64,          
    sender: String,        
    receiver: String,     
    status: TxStatus,
    timestamp: u64,      
}
impl Transaction{
    fn new(txhash: String, amount:f64, sender:String, receiver:String)->Self{
        Transaction{
            txhash,
            amount,
            sender,
            receiver,
            status: TxStatus::Pending(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(), 0.05),
            timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        }
    }

    fn confirm(&mut self, height:u64,miner:String){
        self.status = TxStatus::Confirmed(height, SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs(), miner);
    }
    fn fail(&mut self, err:String,code:u32) {
        self.status = TxStatus::Failed(err, code);
    }
    fn is_completed(&self)->bool{
        match &self.status {
            TxStatus::Pending(time,fee) => {
                println!("时间{}，费用{}", time,fee);
                false
            } 
            TxStatus::Failed(err,val) => {
                println!("原因{}，错误码{}", err,val);
                true
            },
            TxStatus::Confirmed(h, t, m) => {
                println!("高度{}，时间{}，旷工{}", h,t,m);
                true
            }
        }
    }
}
fn main() {
    let mut txs:Vec<Transaction> = Vec::new();
    let mut tx1 = Transaction::new("txhash".to_string(), 1.66,"0xsender1".to_string(),"0xreceiver1".to_string());
    tx1.is_completed();
    tx1.confirm(100,"0xMiner".to_string());
    if tx1.is_completed() {txs.push(tx1)};
    let mut tx2 = Transaction::new("txhash2".to_string(),0.5, "0xsender00".to_string(),"0xreceiver00".to_string());
    tx2.is_completed();
    tx2.fail(String::from("Insuffent balance"),1);
    if tx2.is_completed() { txs.push(tx2)};
    println!("{:#?}", txs);

    println!("{:#?}", txs[1].status);
    if let TxStatus::Failed(reason, code) =&txs[1].status {
         println!("{:#?}", reason);
    }
}
