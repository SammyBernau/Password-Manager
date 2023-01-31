//Useful sources
//https://docs.rs/base64ct/latest/base64ct/#traits
//https://github.com/RustCrypto/AEADs/blob/master/aes-gcm/src/lib.rs

use hex_literal::hex;
use sha2::{Sha256, Digest};
use base64ct::{Base64, Encoding};

pub fn string_hash(input: String) -> Vec<u8> {
    let hash = Sha256::digest(input);

    let mut buf = [0u8; 1024];
    let encoded = Base64::encode(&hash, &mut buf).unwrap();
    let string_vec_hash = encoded.as_bytes().to_vec();

    string_vec_hash
}

