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
    let private_key = generate_private_key();
    println!("{}", slice_to_hex(&private_key));

    let public_key = private_to_public(&private_key);

    // ignore the leading constant `04` byte that signals "no compression"
    let public_key_hashed = keccak(&public_key[1..]);

    // the address is the last 20 bytes of the keccac256 hash of the public key
    let address = &public_key_hashed[12..];
    println!("{}", slice_to_hex(&address));
}

fn slice_to_hex(slice: &[u8]) -> String {
    let mut result = String::new();
    slice.write_hex(&mut result).unwrap();
    result
}

fn keccak(data: &[u8]) -> [u8; 32] {
    let mut sponge = Keccak::new_keccak256();
    sponge.update(data);
    let mut result = [0u8; 32];
    sponge.finalize(&mut result[..]);
    result
}

fn generate_private_key() -> [u8; 32] {
    // a secure random number generator where the random values come directly from the operating system
    let rng = SystemRandom::new();

    let mut private_key_bytes = [0u8; SECRET_KEY_SIZE];
    rng.fill(&mut private_key_bytes[..])
        .expect("system randomness is available; qed");
    private_key_bytes
}

fn private_to_public(private_key_bytes: &[u8; 32]) -> [u8; 65] {
    let curve = Secp256k1::new();
    let private_key = SecretKey::from_slice(&curve, &private_key_bytes[..]).unwrap();
    let public_key = PublicKey::from_secret_key(&curve, &private_key).unwrap();
    let public_key_bytes = public_key.serialize_uncompressed();
    assert_eq!(public_key_bytes[0], 4);
    public_key_bytes
}
