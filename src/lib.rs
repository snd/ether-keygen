extern crate hex;
extern crate ring;
extern crate secp256k1;
extern crate tiny_keccak;

use ring::rand::{SecureRandom, SystemRandom};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use secp256k1::constants::SECRET_KEY_SIZE;
use hex::ToHex;
use tiny_keccak::Keccak;

pub fn slice_to_hex(slice: &[u8]) -> String {
    let mut result = String::new();
    slice.write_hex(&mut result).unwrap();
    result
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut sponge = Keccak::new_keccak256();
    sponge.update(data);
    let mut result = [0u8; 32];
    sponge.finalize(&mut result[..]);
    result
}

pub fn random_private_key() -> [u8; 32] {
    // a secure random number generator where the random values come directly from the operating system
    let rng = SystemRandom::new();

    let mut private_key_bytes = [0u8; SECRET_KEY_SIZE];
    rng.fill(&mut private_key_bytes[..])
        .expect("system randomness is available; qed");
    private_key_bytes
}

pub fn private_to_public(private_key_bytes: &[u8; 32]) -> [u8; 65] {
    let curve = Secp256k1::new();
    let private_key = SecretKey::from_slice(&curve, &private_key_bytes[..]).unwrap();
    let public_key = PublicKey::from_secret_key(&curve, &private_key).unwrap();
    let public_key_bytes = public_key.serialize_uncompressed();
    public_key_bytes
}

pub fn public_to_address(public: &[u8; 65]) -> [u8; 20] {
    // check our assumption that we're dealing with an uncompressed public key
    assert_eq!(public[0], 4);
    // ignore the leading constant `04` byte that signals "no compression"
    let public_key_hashed = keccak256(&public[1..]);
    // the address is the last 20 bytes of the keccac256 hash of the public key
    let mut address = [0u8; 20];
    address.clone_from_slice(&public_key_hashed[12..]);
    address
}

pub fn private_to_address(private: &[u8; 32]) -> [u8; 20] {
    public_to_address(&private_to_public(private))
}

pub fn private_from_hex(string: &str) -> [u8; 32] {
    let mut result = [0u8; 32];
    result.clone_from_slice(&hex::decode(string).unwrap());
    result
}

pub fn address_from_hex(string: &str) -> [u8; 20] {
    let mut result = [0u8; 20];
    result.clone_from_slice(&hex::decode(string).unwrap());
    result
}

#[test]
fn test_private_to_public() {
    assert_eq!(
        address_from_hex("87b85d2cdf2bf4cd1604a6148962b9c4b9c63b40"),
        private_to_address(&private_from_hex(
            "c96f0916fb8ea45d3eecd9c97f8286b0e69728b348eb9c263a06aa16781b5f37"
        ))
    );
    assert_eq!(
        address_from_hex("f13b8f524f02a7efd9cf81ebd02a1ce19061fa20"),
        private_to_address(&private_from_hex(
            "d2cd6ce07b08e0f7bb2cd7cb79ab153b148e353a5d85caf42cfc2d2256e6d9ec"
        ))
    );
    assert_eq!(
        address_from_hex("abdf9e22b4cecb14f1bbb52831a9cb462e34f4be"),
        private_to_address(&private_from_hex(
            "e1c3d9103e74d36958ea0ba93c8556ed857e38178952c355145b76cbd692dc3a"
        ))
    );
}
