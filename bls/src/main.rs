// BLS 简化参数
const P: i64 = 5; // 模数
const N: i64 = 3; // 阶
const G: (i64, i64) = (1, 2); // G 上的基点
const A: i64 = 4; // 曲线参数 a (y^2 = x^3 + ax + b)
const B: i64 = 0; // 曲线参数 b

// 模运算
fn mod_add(a: i64, b: i64, p: i64) -> i64 { (a % p + b % p) % p }
fn mod_sub(a: i64, b: i64, p: i64) -> i64 { (a % p - b % p + p) % p }
fn mod_mul(a: i64, b: i64, p: i64) -> i64 { (a % p * b % p) % p }
fn mod_inv(a: i64, p: i64) -> i64 {
    let mut t0 = 0;
    let mut t1 = 1;
    let mut r0 = p;
    let mut r1 = a % p;
    while r1 != 0 {
        let q = r0 / r1;
        let temp_t = t0 - q * t1;
        t0 = t1;
        t1 = temp_t;
        let temp_r = r0 - q * r1;
        r0 = r1;
        r1 = temp_r;
    }
    if r0 != 1 { return 0; } // 无逆元
    (t0 % p + p) % p
}

// 试探平方根（简化版，仅教学）
fn sqrt_mod(y2: i64, p: i64) -> Option<i64> {
    // 试探法：遍历可能的 y，检查 y^2 == y2
    for y in 0..p {
        if mod_mul(y, y, p) == y2 {
            return Some(y);
        }
    }
    None
}

// 哈希到曲线（修正版）
fn hash_to_curve(m: i64, p: i64) -> Option<(i64, i64)> {
    let x = m % p;
    // y^2 = x^3 + ax + b
    let x2 = mod_mul(x, x, p); // x^2
    let x3 = mod_mul(x, x2, p); // x^3
    let ax = mod_mul(A, x, p); // ax
    let y2 = mod_add(mod_add(x3, ax, p), B, p); // y^2 = x^3 + ax + b
    match sqrt_mod(y2, p) {
        Some(y) => Some((x, y)),
        None => None, // 无平方根，返回 None
    }
}

// 点加法（基于韦伊斯特拉瑟曲线）
fn point_add(p1: Option<(i64, i64)>, p2: Option<(i64, i64)>, a: i64, p: i64) -> Option<(i64, i64)> {
    match (p1, p2) {
        (None, q) => q,
        (p, None) => p,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && mod_add(y1, y2, p) == 0 { return None; } // 相反点
            let m = if x1 == x2 {
                let num = mod_add(mod_mul(3, x2, p), a, p);
                let den = mod_mul(2, y1, p);
                mod_mul(num, mod_inv(den, p), p)
            } else {
                let num = mod_sub(y2, y1, p);
                let den = mod_sub(x2, x1, p);
                mod_mul(num, mod_inv(den, p), p)
            };
            let x3 = mod_sub(mod_sub(mod_mul(m, m, p), x1, p), x2, p);
            let y3 = mod_sub(mod_mul(m, mod_sub(x1, x3, p), p), y1, p);
            Some((x3, y3))
        }
    }
}

// 标量乘法
fn scalar_mul(k: i64, p: Option<(i64, i64)>, a: i64, p_mod: i64) -> Option<(i64, i64)> {
    let mut result = None;
    let mut temp = p;
    for _ in 0..k {
        result = point_add(result, temp, a, p_mod);
    }
    result
}


// 密钥生成
fn keygen() -> (i64, Option<(i64, i64)>) {
    let sk = 1; // 私钥（简化）
    let pk = scalar_mul(sk, Some(G), A, P); // 公钥 P = [sk]G
    (sk, pk)
}

// 签名生成
fn sign(m: i64, sk: i64) -> Option<(i64, i64)> {
    match hash_to_curve(m, P) {
        Some(h) => scalar_mul(sk, Some(h), A, P), // S = [sk]H(m)
        None => None,
    }
}

// 验证
fn verify(m: i64, sig: Option<(i64, i64)>, pk: Option<(i64, i64)>) -> bool {
    match (sig, pk, hash_to_curve(m, P)) {
        (Some(s), Some(p), Some(h)) => {
            let left = pairing(G, s, P); // e(G, S)
            let right = pairing(p, h, P); // e(P, H(m))
            left == right
        }
        _ => false,
    }
}
// 简化配对运算（仅示意）
fn pairing(p: (i64, i64), q: (i64, i64), p_mod: i64) -> i64 {
    mod_mul(p.0, q.0, p_mod) // 简化：用 x 坐标相乘代替配对
}

fn main() {
    let (sk, pk) = keygen();
    let m = 1; // 消息
    let sig = sign(m, sk);
    let is_valid = verify(m, sig, pk);
    println!("私钥: {}", sk);
    println!("公钥: {:?}", pk);
    println!("签名: {:?}", sig);
    println!("验证结果: {}", is_valid);
}