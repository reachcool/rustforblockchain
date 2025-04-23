use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::any::type_name;
fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}
fn main() {
    println!("开始猜数!");
    let rand_number = rand::thread_rng().gen_range(1000..=9999);
    print_type_of(&rand_number); 
    loop {
        println!("请输入你猜的数.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("获取终端输入失败");

        println!("你输入的数是: {}", guess);
        let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue,
            };
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("小了!"),
            Ordering::Greater => println!("大了!"),
            Ordering::Equal => {
                println!("祝贺你，猜对了!");
                break;
            }
        }
    } 
}
