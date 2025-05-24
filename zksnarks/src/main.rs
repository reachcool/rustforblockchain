// 简化参数
const P: i64 = 17; // 模数
const N: i64 = 3; // 阶
const G: (i64, i64) = (1, 2); // 唯一基点
const A: i64 = 0; // 曲线参数 a
const B: i64 = 1; // 曲线参数 b

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
    if r0 != 1 { return 0; }
    (t0 % p + p) % p
}

// 点加法
fn point_add(p1: Option<(i64, i64)>, p2: Option<(i64, i64)>, a: i64, p: i64) -> Option<(i64, i64)> {
    match (p1, p2) {
        (None, q) => q,
        (p, None) => p,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && mod_add(y1, y2, p) == 0 { return None; }
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

// 标量乘法
fn scalar_mul(k: i64, p: Option<(i64, i64)>, a: i64, p_mod: i64) -> Option<(i64, i64)> {
    let mut result = None;
    let mut temp = p;
    let mut k = k % N;
    while k > 0 {
        if k % 2 == 1 {
            result = point_add(result, temp, a, p_mod);
        }
        temp = point_add(temp, temp, a, p_mod);
        k /= 2;
    }
    result
}

// 哈希函数（简化）
fn hash(x: i64, y: i64) -> i64 {
    (x + y) % P // 模拟多项式哈希
}

// 密钥生成（Trusted Setup）
fn setup() -> (Option<(i64, i64)>, Option<(i64, i64)>) {
    let s = 2; // 秘密值
    let pk = scalar_mul(s, Some(G), A, P); // 证明密钥 [s]G
    let vk = scalar_mul(1, Some(G), A, P); // 验证密钥 [1]G
    (pk, vk)
}

// 证明生成（验证 x + y = z）
fn prove(x: i64, y: i64, z: i64, pk: Option<(i64, i64)>) -> (Option<(i64, i64)>, i64) {
    if mod_add(x, y, P) != z % P { return  (None,0); }
    let h = hash(x, y);
    if pk.is_none() { return (None,h); }
    let proof = scalar_mul(h, pk, A, P); // 证明点 [h][s]G
    (proof,h)
}

// 验证
fn verify(h:i64, proof: Option<(i64, i64)>, vk: Option<(i64, i64)>) -> bool {
    match (proof, vk) {
        (Some(p), Some(v)) => {
            let s = 2; // 硬编码 s，与 setup 中的 s 一致
            let expected = scalar_mul(h, Some(v), A, P); // [h]vk
            match expected {
                Some(e) => {
                    // [s][h]vk
                    let scaled_vk = scalar_mul(s, Some(e), A, P); 
                    match scaled_vk {
                        Some(sv) => sv == p, // [s][h]vk == proof
                        None => false,
                    }
                }
                None => false,
            }
        }
        _ => false,
    }
}

fn main() {
    let (pk, vk) = setup();
    let x = 2;
    let y = 3;
    let z = 5; // x + y = z
    let proof = prove(x, y, z, pk);
    let is_valid = verify(proof.1, proof.0, vk);
    println!("证明密钥: {:?}", pk);
    println!("验证密钥: {:?}", vk);
    println!("证明: {:?}", proof);
    println!("验证结果: {}", is_valid);
}