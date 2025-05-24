fn main() {
    // 明文：16字节，从0到15
    let plaintext = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    // 密钥：16字节，全0
    let key = [0; 16];

    println!("Plaintext: {:?}", plaintext);
    print_state(&plaintext, "Initial State");
    let ciphertext = aes_encrypt(&plaintext, &key);
    println!("Ciphertext: {:?}", ciphertext);
    print_state(&ciphertext, "After Encryption");
    let decrypted = aes_decrypt(&ciphertext, &key);
    println!("Decrypted: {:?}", decrypted);
    print_state(&decrypted, "After Decryption");
}

// 打印4x4状态矩阵
fn print_state(state: &[u8; 16], label: &str) {
    println!("\n{}:", label);
    for i in 0..4 {
        println!(
            "Row {}: [{:2}, {:2}, {:2}, {:2}]",
            i,
            state[i],
            state[i + 4],
            state[i + 8],
            state[i + 12]
        );
    }
}

// 子字节替换：每个字节加1（简化的S盒）
fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = byte.wrapping_add(1);
    }
}

// 逆子字节替换：每个字节减1
fn inv_sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = byte.wrapping_sub(1);
    }
}

// 行移位：按行处理，第i行左移i位
fn shift_rows(state: &mut [u8; 16]) {
    let s = *state;
    // 第0行：不动 [s[0], s[4], s[8], s[12]]
    state[0] = s[0];    state[4] = s[4];    state[8] = s[8];    state[12] = s[12];
    // 第1行：左移1位 [s[1], s[5], s[9], s[13]] -> [s[5], s[9], s[13], s[1]]
    state[1] = s[5];    state[5] = s[9];    state[9] = s[13];    state[13] = s[1];
    // 第2行：左移2位 [s[2], s[6], s[10], s[14]] -> [s[10], s[14], s[2], s[6]]
    state[2] = s[10];    state[6] = s[14];    state[10] = s[2];    state[14] = s[6];
    // 第3行：左移3位 [s[3], s[7], s[11], s[15]] -> [s[15], s[3], s[7], s[11]]
    state[3] = s[15];    state[7] = s[3];    state[11] = s[7];    state[15] = s[11];
}

// 逆行移位：按行处理，第i行右移i位
fn inv_shift_rows(state: &mut [u8; 16]) {
    let s = *state;
    // 第0行：不动 [s[0], s[4], s[8], s[12]]
    state[0] = s[0];    state[4] = s[4];    state[8] = s[8];    state[12] = s[12];
    // 第1行：右移1位 [s[1], s[5], s[9], s[13]] -> [s[13], s[1], s[5], s[9]]
    state[1] = s[13];    state[5] = s[1];    state[9] = s[5];    state[13] = s[9];
    // 第2行：右移2位 [s[2], s[6], s[10], s[14]] -> [s[10], s[14], s[2], s[6]]
    state[2] = s[10];    state[6] = s[14];    state[10] = s[2];    state[14] = s[6];
    // 第3行：右移3位 [s[3], s[7], s[11], s[15]] -> [s[7], s[11], s[15], s[3]]
    state[3] = s[7];    state[7] = s[11];    state[11] = s[15];    state[15] = s[3];
}

// 列混淆：每字节与固定偏移异或
fn mix_columns(state: &mut [u8; 16]) {
    for i in 0..16 {
        // 每个字节与索引异或，模拟扩散
        state[i] = state[i] ^ (i as u8);
    }
}

// 逆列混淆：同正向（异或自逆）
fn inv_mix_columns(state: &mut [u8; 16]) {
    mix_columns(state); // 异或自逆
}

// 轮密钥加：状态与密钥异或
fn add_round_key(state: &mut [u8; 16], key: &[u8; 16]) {
    for i in 0..16 {
        state[i] ^= key[i];
    }
}

// AES加密：单轮变换
fn aes_encrypt(plaintext: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let mut state = *plaintext;
    print_state(&state, "Before SubBytes");
    sub_bytes(&mut state);
    print_state(&state, "After SubBytes");
    shift_rows(&mut state);
    print_state(&state, "After ShiftRows");
    mix_columns(&mut state);
    print_state(&state, "After MixColumns");
    add_round_key(&mut state, key);
    print_state(&state, "After AddRoundKey");
    state
}

// AES解密：单轮逆变换
fn aes_decrypt(ciphertext: &[u8; 16], key: &[u8; 16]) -> [u8; 16] {
    let mut state = *ciphertext;
    print_state(&state, "Before Inv AddRoundKey");
    add_round_key(&mut state, key);
    print_state(&state, "After Inv AddRoundKey");
    inv_mix_columns(&mut state);
    print_state(&state, "After Inv MixColumns");
    inv_shift_rows(&mut state);
    print_state(&state, "After Inv ShiftRows");
    inv_sub_bytes(&mut state);
    print_state(&state, "After Inv SubBytes");
    state
}