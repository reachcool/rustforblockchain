use rand::Rng;
use std::cmp::Ordering;

fn shamir_split(secret: i64, num_shares: usize, threshold: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut shares = vec![0; num_shares];
    let mut coeffs = vec![secret]; // 常数项是秘密

    // 生成较小的整数随机系数
    for _ in 1..threshold {
        coeffs.push(rng.gen_range(-5..5)); // 使用整数系数
    }

    // 计算每个份额
    for x in 1..=num_shares {
        let mut share = 0;
        for (power, &coeff) in coeffs.iter().enumerate() {
            share += coeff * (x as i64).pow(power as u32);
            println!("coeff: {}, x: {}, power: {}, share: {}", coeff, x, power, share);
        }
        shares[x - 1] = share;
    }
    println!("shares: {:?}", shares);
    shares
}

fn shamir_reconstruct(shares: Vec<i64>, xs: Vec<i64>) -> i64 {
    let mut secret = 0;
    for (i, &share) in shares.iter().enumerate() {
        println!("i and share: {}, {}", i, share);
        let mut num = 1; // 分子
        let mut den = 1; // 分母
        for (j, &xj) in xs.iter().enumerate() {
            if i != j {
                // 拉格朗日插值：(-xj) / (xs[i] - xj)
                num *= -xj; // 分子累积：-xj
                den *= xs[i] - xj; // 分母累积：xs[i] - xj
                println!("j: {}, xj: {}, num: {}, den: {}", j, xj, num, den);
            }
        }
        // 分数计算：share * num / den
        let term = (share * num) / den;
        println!("term: {}", term);
        secret += term;
    }
    println!("reconstructed secret: {}", secret);
    secret
}

fn mpc_compare(secret_a: i64, secret_b: i64) -> Ordering {
    let shares_a = shamir_split(secret_a, 2, 2);
    let shares_b = shamir_split(secret_b, 2, 2);

    // 模拟交换：A的第一个份额给B，B的第一个份额给A
    let a_share_to_b = shares_a[0];
    let b_share_to_a = shares_b[0];

    // 重构值
    let reconstructed_a = shamir_reconstruct(vec![shares_a[1], a_share_to_b], vec![1, 2]);
    let reconstructed_b = shamir_reconstruct(vec![shares_b[1], b_share_to_a], vec![1, 2]);

    // 计算差值
    let diff = reconstructed_a - reconstructed_b;
    println!("reconstructed_a: {}, reconstructed_b: {}, diff: {}", reconstructed_a, reconstructed_b, diff);

    // 根据差值判断大小
    diff.cmp(&0)
}

fn main() {
    let wealth_a = 1000; // 参与方A的财富
    let wealth_b = 1500; // 参与方B的财富

    match mpc_compare(wealth_a, wealth_b) {
        Ordering::Less => println!("A is less wealthy than B"),
        Ordering::Equal => println!("A and B have equal wealth"),
        Ordering::Greater => println!("A is wealthier than B"),
    }
}