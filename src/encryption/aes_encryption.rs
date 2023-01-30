use std::borrow::Borrow;
use std::str::from_utf8;
use aead::AeadCore;
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce // Or `Aes128Gcm`
};
use cipher::generic_array::GenericArray;
use rand::distributions::uniform::SampleBorrow;
use cipher::consts::U16;
use generic_array::ArrayLength;
use rand::prelude::*;
use sha2::{Sha256, Digest};

pub fn aes_string_encryption(key_str: String, message: String) -> (Vec<u8>, Vec<u8>) {
    //Hash the string key to create a fixed-length key
    let mut hasher = Sha256::digest(key_str.as_bytes());
    let key = hasher.as_slice();

    let aead = Aes256Gcm::new(GenericArray::from_slice(key));

    //Only 4,294,967,296 messages with random nonces can be encrypted under a given key. Something to think about but for the purpose of this project doesn't matter a whole lot
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher_text = aead.encrypt(nonce.clone().borrow(), message.as_bytes().as_ref()).expect("ENCRYPTION FAILURE!");

    (cipher_text,nonce.to_vec())
}

pub fn aes_string_decryption(key_str: String,  cipher_text: Vec<u8>, nonce: Vec<u8>) ->Vec<u8> {
    //Hash the string key to create a fixed-length key
    let mut hasher = Sha256::digest(key_str.as_bytes());
    let key = hasher.as_slice();

    let aead = Aes256Gcm::new(GenericArray::from_slice(key));

    let plain_text = aead.decrypt(GenericArray::from_slice(&*nonce), cipher_text.as_ref()).expect("DECRYPTION FAILURE!");

    plain_text
}
