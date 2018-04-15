extern crate secp256k1;
extern crate ring;
extern crate hex;
extern crate tiny_keccak;

use ring::rand::{SystemRandom, SecureRandom};
use secp256k1::{SecretKey, Secp256k1, PublicKey};
use secp256k1::constants::SECRET_KEY_SIZE;
use hex::ToHex;
use tiny_keccak::Keccak;

fn main() {
    // a secure random number generator where the random values come directly from the operating system
    let rng = SystemRandom::new();

    let curve = Secp256k1::new();

    let mut private_key_bytes = [0u8; SECRET_KEY_SIZE];
    rng.fill(&mut private_key_bytes[..])
        .expect("system randomness is available; qed");
    let private_key = SecretKey::from_slice(&curve, &private_key_bytes[..]).unwrap();

    let mut private_key_hex = String::new();
    private_key_bytes.write_hex(&mut private_key_hex).unwrap();
    println!("{}", private_key_hex);

    let public_key = PublicKey::from_secret_key(&curve, &private_key).unwrap();
    let public_key_bytes = public_key.serialize_uncompressed();

    assert_eq!(public_key_bytes[0], 4);

    let mut keccac = Keccak::new_keccak256();
    // ignore the leading constant `04` byte that signals "no compression"
    keccac.update(&public_key_bytes[1..]);
    let mut public_key_hash = [0u8; 32];
    keccac.finalize(&mut public_key_hash[..]);

    let mut public_key_hex = String::new();
    public_key_hash.write_hex(&mut public_key_hex).unwrap();

    let address_bytes = &public_key_hash[12..];
    let mut address_hex = String::new();
    address_bytes.write_hex(&mut address_hex).unwrap();
    println!("{}", address_hex);
}
