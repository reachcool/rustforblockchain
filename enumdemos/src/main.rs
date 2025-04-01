#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    // --省略其他州名称--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),

}
fn main() {
    println!("Hello Match!");
    println!("这个硬币面值为:{}", value_of_coin(Coin::Quarter(UsState::Alabama)));    
}
fn value_of_coin(coin: Coin)-> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny Coin");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("这枚硬币上印的是{state:?}州");
            25
        }
    }
}
