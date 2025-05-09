use ring::rand::SystemRandom;
use ring::signature::{
    EcdsaKeyPair, KeyPair, UnparsedPublicKey,
    ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING,
};

struct RingSignature {
    pubkeys: Vec<Vec<u8>>,
    signature: Vec<u8>,
}

fn generate_ring_signature(
    keypair: &EcdsaKeyPair,
    pubkeys: &[Vec<u8>],
    message: &[u8],
    rng: &SystemRandom,
) -> RingSignature {
    let sig = keypair
        .sign(rng, message)
        .expect("Failed to sign message");
    RingSignature {
        pubkeys: pubkeys.to_vec(),
        signature: sig.as_ref().to_vec(),
    }
}

fn verify_ring_signature(ring_sig: &RingSignature, message: &[u8]) -> bool {
    for pubkey in &ring_sig.pubkeys {
        let verifier = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, pubkey);
        if verifier.verify(message, &ring_sig.signature).is_ok() {
            return true;
        }
    }
    false
}

fn main() {
    let rng = SystemRandom::new();

    // 生成 ECDSA 密钥对
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(
        &ECDSA_P256_SHA256_ASN1_SIGNING,
        &rng,
    )
    .expect("Failed to generate keypair");

    let keypair = EcdsaKeyPair::from_pkcs8(
        &ECDSA_P256_SHA256_ASN1_SIGNING,
        pkcs8_bytes.as_ref(),
        &rng,
    )
    .expect("Failed to parse keypair");

    // 模拟一个环，仅包含自己的公钥
    let pubkeys = vec![keypair.public_key().as_ref().to_vec()];
    let message = b"Test transaction";

    let ring_sig = generate_ring_signature(&keypair, &pubkeys, message, &rng);
    let is_valid = verify_ring_signature(&ring_sig, message);

    println!("✅ Ring Signature Valid: {}", is_valid);
}
