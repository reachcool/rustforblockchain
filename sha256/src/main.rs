// SHA-256 实现
use std::fmt::Write;

// 优化后的右循环移位函数：确保 shift 在 0~31 范围内，避免不必要计算
fn right_rotate(x: u32, n: u32) -> u32 {
    let n = n & 31; // 限制 shift 范围，32 位整数移位只需 0~31
    (x >> n) | (x << (32 - n))
}

// SHA-256 初始哈希值（H0~H7，基于标准）
const INITIAL_HASH: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// SHA-256 标准 K 表（64 个常量，基于前 64 个素数的立方根小数部分）
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// SHA-256 实现：输入为字符串，返回 256 位（32 字节）哈希值
fn sha256(input: &str) -> [u8; 32] {
    // 初始化哈希状态
    let mut h = INITIAL_HASH;

    // 步骤 1：消息转字节，填充
    let mut msg = input.as_bytes().to_vec();
    let bit_len = (input.len() as u64) * 8;

    // 添加 "1" 位（0x80）
    msg.push(0x80);

    // 填充 0，直到长度满足 (len + 8) % 64 == 0
    // 目标：预留 8 字节（64 位）给长度字段
    while msg.len() % 64 != 56 {
        msg.push(0x00);
    }

    // 添加 64 位长度（大端序）
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // 步骤 2：处理每个 512 位块（64 字节）
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];

        // 2.1 填充 w[0..15]：每 4 字节转为一个 u32（大端序）
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }

        // 2.2 扩展 w[16..63]
        for i in 16..64 {
            let s0 = right_rotate(w[i - 15], 7) ^ right_rotate(w[i - 15], 18) ^ (w[i - 15] >> 3);
            let s1 = right_rotate(w[i - 2], 17) ^ right_rotate(w[i - 2], 19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        // 2.3 初始化工作变量
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_val = h[7];

        // 2.4 64 轮压缩函数
        for i in 0..64 {
            let s1 = right_rotate(e, 6) ^ right_rotate(e, 11) ^ right_rotate(e, 25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h_val
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = right_rotate(a, 2) ^ right_rotate(a, 13) ^ right_rotate(a, 22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h_val = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        // 2.5 更新哈希状态
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_val);
    }

    // 步骤 3：将哈希值转为字节数组
    let mut result = [0u8; 32];
    for (i, &word) in h.iter().enumerate() {
        result[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
    }
    result
}

fn main() {
    let input = "hello, world";
    let hash = sha256(input);
    
    // 格式化输出为标准十六进制字符串
    let mut hash_hex = String::with_capacity(64);
    for byte in hash {
        write!(&mut hash_hex, "{:02x}", byte).unwrap();
    }
    
    println!("SHA-256 hash of '{}': {}", input, hash_hex);
}