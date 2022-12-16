use crypto::{aead::AeadEncryptor};
use rustc_serialize::hex::FromHex;
use core::str;
use std::iter::repeat;
use hex;








fn hex_to_bytes(s: String) -> Vec<u8> {
    s.from_hex().unwrap()
}

fn key_pad(mut s: String) -> String{
    while s.len() < 64 {
        s+="0";
    }
    s
}

fn iv_pad(mut s: String) -> String {
    while s.len() < 16 {
        s+="0";
    }
    s
}

pub fn get_iv(user_password: &String) -> String {
    //Uses first, middle and last char of password to create the first 3 characters for iv
    //num_string.chars().nth(i).unwrap()

    let first_char = user_password.chars().nth(0).unwrap();

    let end_char = user_password.chars().nth(user_password.len()-1).unwrap();

    let middle_char = user_password.chars().nth(get_str_middle_char(user_password)).unwrap();

    let mut iv_start = "".to_string();
    iv_start.push(first_char);
    iv_start.push(end_char);
    iv_start.push(middle_char);

    let final_iv_start = hex::encode(iv_start);

    let iv = iv_pad(final_iv_start);

    iv
}

fn get_str_middle_char(s: &str)->usize{
    let position;
    if s.len() % 2 == 1 {
        position = s.len() /2;
    } else {
        position = s.len()/2-1;
    }
    position
}


pub fn chacha20poly1305_encrypt_string(master_password: String, msg_input: String) -> (Vec<u8>, Vec<u8>) {
    let my_iv = get_iv(&master_password);
    let key = key_pad(hex::encode(master_password));
    let msg = msg_input;
    let my_add ="Dogs are cool";

    let key=&hex_to_bytes(key)[..];
    let iv=&hex_to_bytes(my_iv)[..];
    let plain=msg.as_bytes();
    let aad= my_add.as_bytes();

// Encrypting
    let mut encrypt_cipher = crypto::chacha20poly1305::ChaCha20Poly1305::new(key, iv, aad);
    let mut output: Vec<u8> = repeat(0).take(plain.len()).collect();
    let mut outtag: Vec<u8> = repeat(0).take(16).collect();
    encrypt_cipher.encrypt(plain, &mut output[..], &mut outtag[..]);

    return (output, outtag);
}

pub fn chacha20poly1305_decrypt_string(master_password: String, encrypted_output:Vec<u8>, mut encrypted_string_outtag: Vec<u8>) ->Vec<u8>{
    let my_iv =get_iv(&master_password);
    let key = key_pad(hex::encode(master_password));
    let my_add ="Dogs are cool";

    let key=&hex_to_bytes(key)[..];
    let iv=&hex_to_bytes(my_iv)[..];
    let aad= my_add.as_bytes();

    let mut decrypt_cipher = crypto::chacha20poly1305::ChaCha20Poly1305::new(key, iv, aad);
    let mut newoutput: Vec<u8> = repeat(0).take(encrypted_output.len()).collect();
    decrypt_cipher.encrypt(&encrypted_output[..], &mut newoutput[..], &mut encrypted_string_outtag[..]);


    return newoutput;
}