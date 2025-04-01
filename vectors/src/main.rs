fn main() {
    // println!("Hello, Vectors!");
    // let v = vec![1, 2, 3, 4, 5];

    // let third: i32 = v[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    let mut users: Vec<User> = Vec::new();
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        wallet_address: String,
    }
    let user1 = User {
        active: true,
        username: String::from("user1name"),
        email: String::from("user1@structexample.cn"),
        wallet_address: String::from("0xAddress"),
    };
    let user2 = User {
        active:true,
        username: user1.username.clone(),
        email: user1.email.clone(),
        wallet_address:String::from("0xuser2"),
    };
    users.push(user1);
    users.push(user2);

    // let user11 = users.get(0);
    let user22 = users.get_mut(1);
    match user22 {
        Some(user) => {println!("before {:#?}", user);
            user.wallet_address = String::from("0x0002");
            println!("after {:#?}",user);
        },

        None => println!("There is no third element."),
    }
   
}
