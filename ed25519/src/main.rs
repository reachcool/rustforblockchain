// 椭圆曲线参数（简化版）
const P: i64 = 5; // 模数
const D: i64 = 1; // 曲线参数 d
const L: i64 = 3; // 阶
const B: (i64, i64) = (1, 2); // 基点 B

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
    (t0 % p + p) % p
}

// 点加法（简化版）
fn point_add(p1: Option<(i64, i64)>, p2: Option<(i64, i64)>, d: i64, p: i64) -> Option<(i64, i64)> {
    match (p1, p2) {
        (None, q) => q,
        (p, None) => p,
        (Some((x1, y1)), Some((x2, y2))) => {
            let num = mod_sub(y2, x1 * y1 * x2 * y2 * d, p);
            let den = mod_sub(x2, x1 * y1 * y2, p);
            let m = mod_mul(num, mod_inv(den, p), p);
            let x3 = mod_sub(mod_mul(m, m, p), mod_add(x1, x2, p), p);
            let y3 = mod_sub(mod_mul(m, mod_sub(x1, x3, p), p), y1, p);
            Some((x3, y3))
        }
    }
}

// 标量乘法
fn scalar_mul(k: i64, p: Option<(i64, i64)>, d: i64, p_mod: i64) -> Option<(i64, i64)> {
    let mut result = None;
    let mut temp = p;
    for _ in 0..k {
        result = point_add(result, temp, d, p_mod);
    }
    result
}

// 哈希函数（简化为模运算）
fn hash(data: i64, p: i64) -> i64 { data % p }

// 签名
fn sign(message: i64, secret: i64) -> ((i64, i64), i64) {
    let r = hash(hash(secret, L) + message, L); // 确定性 nonce
    let r_point = scalar_mul(r, Some(B), D, P);
    let a = secret % L; // 私钥
    let h = hash(hash(r_point.unwrap().0, L) + hash(a, L) + message, L);
    let s = (r + mod_mul(h, a, L)) % L;
    (r_point.unwrap(), s)
}

// 验证
fn verify(message: i64, signature: ((i64, i64), i64), public_key: (i64, i64)) -> bool {
    let ((rx, ry), s) = signature;
    let r_point = Some((rx, ry));
    let h = hash(hash(rx, L) + hash(public_key.0, L) + message, L);
    let left = scalar_mul(s, Some(B), D, P);
    let right = point_add(r_point, scalar_mul(mod_mul(h, public_key.0, L), Some(public_key), D, P), D, P);
    left == right
}

fn main() {
    let secret = 1; // 私钥
    let public_key = scalar_mul(secret, Some(B), D, P).unwrap();
    let message = 1;
    let signature = sign(message, secret);
    let is_valid = verify(message, signature, public_key);
    println!("公钥: {:?}", public_key);
    println!("签名: {:?}", signature);
    println!("验证结果: {}", is_valid);
}