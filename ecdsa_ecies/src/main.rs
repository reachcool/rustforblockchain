// 椭圆曲线参数
const P: i64 = 5; // 模数
const A: i64 = 2; // 曲线 y^2 = x^3 + ax + b 中的 a
const B: i64 = 1; // 曲线中的 b
const N: i64 = 7; // 生成点的阶
const G: (i64, i64) = (1, 2); // 生成点 G

// 模运算：加、减、乘、逆
fn mod_add(a: i64, b: i64, p: i64) -> i64 {
    (a % p + b % p) % p
}

fn mod_sub(a: i64, b: i64, p: i64) -> i64 {
    (a % p - b % p + p) % p
}

fn mod_mul(a: i64, b: i64, p: i64) -> i64 {
    (a % p * b % p) % p
}

fn mod_inv(a: i64, p: i64) -> i64 {
    let mut t0 = 0i64;
    let mut t1 = 1i64;
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

// 点加法
fn point_add(p1: Option<(i64, i64)>, p2: Option<(i64, i64)>, a: i64, p: i64) -> Option<(i64, i64)> {
    match (p1, p2) {
        (None, q) => q,
        (p, None) => p,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && mod_add(y1, y2, p) == 0 { None } // 相反点
            else {
                let m = if x1 == x2 {
                    let num = mod_add(mod_mul(3, mod_mul(x1, x1, p), p), a, p);
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

// 签名
fn sign(m: i64, d: i64) -> (i64, i64) {
    let k = 2; // 随机数 k
    let e = m % N; // 简化的哈希
    let R = scalar_mul(k, Some(G), A, P).unwrap();
    let r = R.0 % N;
    let s = mod_mul(mod_inv(k, N), mod_add(e, mod_mul(d, r, N), N), N);
    if s == 0 {
        panic!("s 不能为 0，需重新选择 k");
    }
    (r, s)
}

// 验证
fn verify(m: i64, sig: (i64, i64), Q: Option<(i64, i64)>) -> bool {
    let (r, s) = sig;
    let e = m % N;
    let w = mod_inv(s, N);
    let u1 = mod_mul(e, w, N);
    let u2 = mod_mul(r, w, N);
    let U = point_add(scalar_mul(u1, Some(G), A, P), scalar_mul(u2, Q, A, P), A, P);
    U.map_or(false, |u| u.0 % N == r)
}

fn main() {
    // 密钥生成
    let d = 1; // 私钥
    let Q = scalar_mul(d, Some(G), A, P); // 公钥
    println!("私钥 d: {}", d);
    println!("公钥 Q: {:?}", Q);

    // 签名
    let m = 1; // 消息
    let (r, s) = sign(m, d);
    println!("签名 (r, s): ({}, {})", r, s);

    // 验证
    let valid = verify(m, (r, s), Q);
    println!("签名有效: {}", valid);
}