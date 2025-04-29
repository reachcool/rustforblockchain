use bitcoin::{Address, Network, PublicKey, ScriptBuf};
use bitcoin::blockdata::script::Builder;
use secp256k1::{Secp256k1, SecretKey};
use rand::rngs::OsRng;

fn generate_keypair(secp: &Secp256k1<secp256k1::All>) -> (SecretKey, PublicKey) {
    let secret_key = SecretKey::new(&mut OsRng);
    let public_key = PublicKey::new(secp256k1::PublicKey::from_secret_key(secp, &secret_key));
    (secret_key, public_key)
}

fn create_multisig_address(pubkeys: &[PublicKey], m: usize, network: Network) -> (Address, ScriptBuf) {
    let redeem_script = {
        let mut builder = Builder::new()
            .push_int(m as i64);
        for pk in pubkeys {
            builder = builder.push_key(pk);
        }
        builder
            .push_int(pubkeys.len() as i64)
            .push_opcode(bitcoin::blockdata::opcodes::all::OP_CHECKMULTISIG)
            .into_script()
    };

    let address = Address::p2sh(&redeem_script, network)
        .expect("Failed to create P2SH address");

    (address, redeem_script)
}


fn main() {
    let secp = Secp256k1::new();

    let mut pubkeys = vec![];
    for _ in 0..3 {
        let (_sk, pk) = generate_keypair(&secp);
        pubkeys.push(pk);
    }

    let (address, redeem_script) = create_multisig_address(&pubkeys, 2, Network::Bitcoin);

    println!("Multisig Address: {}", address);
    println!("Redeem Script (hex): {}", redeem_script.to_bytes().iter().map(|b| format!("{:02x}", b)).collect::<String>());
}
