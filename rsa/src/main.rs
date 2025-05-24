// 简化的 RSA 算法实现

// 模幂运算
fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

// 模逆（扩展欧几里得算法）
fn mod_inverse(e: u64, phi: u64) -> Option<u64> {
    let mut a = e as i64;
    let mut b = phi as i64;
    let mut x = 1;
    let mut y = 0;

    while b != 0 {
        let quotient = a / b;
        let temp = b;
        b = a % temp;
        a = temp;

        let temp_x = x;
        x = y;
        y = temp_x - quotient * y;
    }

    if a != 1 {
        return None; // GCD ≠ 1，无模逆
    }

    // 确保结果为正
    let result = if x < 0 {
        x + phi as i64
    } else {
        x
    };

    Some(result as u64)
}

// 生成 RSA 密钥对
fn generate_keypair(p: u64, q: u64) -> ((u64, u64), (u64, u64)) {
    let n = p * q;
    let phi = (p - 1) * (q - 1);
    let e = 17; // 与 phi 互质（GCD(17, 3120) = 1）
    let d = mod_inverse(e, phi).expect("模逆不存在");

    ((n, e), (n, d)) // 公钥 (n, e)，私钥 (n, d)
}

fn main() {
    // 素数
    let p = 61;
    let q = 53;
    let ((n, e), (_, d)) = generate_keypair(p, q);
    println!("({n},{e}),({n},{d})");
    // 明文（需小于 n）
    let message: u64 = 42;
    println!("原始消息: {}", message);

    // 加密：C = M^e mod n
    let ciphertext = mod_pow(message, e, n);
    println!("密文: {}", ciphertext);

    // 解密：M = C^d mod n
    let decrypted = mod_pow(ciphertext, d, n);
    println!("解密消息: {}", decrypted);

    // 验证
    assert_eq!(message, decrypted);
}