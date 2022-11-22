//Both examples but second link looks better
//https://asecuritysite.com/symmetric/rust_chacha20poly
//https://medium.com/asecuritysite-when-bob-met-alice/the-near-perfect-encryption-method-and-the-best-programming-lanaguage-meet-aes-gcm-and-rust-33ef68c06677

//TODO: Pretty sure i need a file encryption function not a string encryption function. Sooooo look at this link to do file encryption
//https://github.com/skerkour/kerkour.com/blob/main/2021/rust_file_encryption/src/main.rs
use crypto::{aead::AeadEncryptor, symmetriccipher::{ SynchronousStreamCipher}};
use rustc_serialize::hex::FromHex;
use std::env;
use core::str;
use std::iter::repeat;
use hex;




use chacha20poly1305::{
    aead::{Aead, AeadCore,AeadInPlace, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce
};

use crypto::aes_gcm::AesGcm;
use crypto::aead::{AeadDecryptor};

fn hex_to_bytes(s: String) -> Vec<u8> {
    s.from_hex().unwrap()
}

fn key_pad(mut s: String) -> String{
    while s.len() < 64 {
        s+="0";
    }
    return s;
}

fn iv_pad(mut s: String) -> String {
    while s.len() < 16 {
        s+="0";
    }
    return s;
}

pub fn get_iv(user_password: &str) -> String {
    //Uses first, middle and last char of password to create the first 3 characters for iv
    //num_string.chars().nth(i).unwrap()

    let first_char = user_password.chars().nth(0).unwrap();

    let end_char = user_password.chars().nth(user_password.len()-1).unwrap();

    let middle_char = user_password.chars().nth(get_str_middle_char(user_password)).unwrap();

    let mut iv_start = "".to_string();
    iv_start.push(first_char); 
    iv_start.push(end_char);
    iv_start.push(middle_char);

    let mut final_iv_start = hex::encode(iv_start);

    let iv = iv_pad(final_iv_start);

    return iv;
}

fn get_str_middle_char(s: &str)->usize{
    let position;
    if s.len() % 2 == 1 {
        position = s.len() /2;
    } else {
        position = s.len()/2-1;
    }
    return position;
}


pub fn chacha20poly1305_encrypt(user_password: &str, mut msg_input: &str) {
    let mut key = hex::encode(user_password);
    key = key_pad(key);
    let mut msg = msg_input;
    let mut my_iv =get_iv(user_password);
    let my_add ="lots of cum";
    let args: Vec<String> = env::args().collect();

    if args.len() >1 { msg = args[1].as_str();}
    if args.len() >2 { key = args[2].as_str().parse().unwrap();}
    if args.len() >3 { my_iv = args[3].as_str().parse().unwrap();}
    // println!("== ChaCha20/Poly1305 ==");
    // println!("Message: {:?}",msg);
    // println!("Key: {:?}", key);
    // println!("IV: {:?}", my_iv);
    // println!("Additional data: {:?}", my_add);
    let key=&hex_to_bytes(key)[..];
    let iv=&hex_to_bytes(my_iv)[..];
    let plain=msg.as_bytes();
    let aad= my_add.as_bytes();

// Encrypting
    let mut encrypt_cipher = crypto::chacha20poly1305::ChaCha20Poly1305::new(&key, iv, aad);
    let mut output: Vec<u8> = repeat(0).take(plain.len()).collect();
    let mut outtag: Vec<u8> = repeat(0).take(16).collect();
    encrypt_cipher.encrypt(&plain[..], &mut output[..], &mut outtag[..]);
    // println!("\nEncrypted: {}",hex::encode(output.clone()));
    // println!("\nTag: {}",hex::encode(outtag.clone()));

    let mut decrypt_cipher = crypto::chacha20poly1305::ChaCha20Poly1305::new(&key, iv, aad);
    let mut newoutput: Vec<u8> = repeat(0).take(output.len()).collect();
    decrypt_cipher.encrypt(&mut output.clone()[..], &mut newoutput[..], &mut outtag[..]);
    // println!("\nDecrypted: {}",str::from_utf8(&newoutput[..]).unwrap());
}


// pub fn aes_gcm_example(){
//     let mut mykey="00000000000000000000000000000004";
//     let mut msg="hello";
//     let myadd="Additional Data for authentication!";
//
//     let mut myiv="000000000000000000000001";
//
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() >1 { msg = args[1].as_str();}
//     if args.len() >2 { mykey = args[2].as_str();}
//     if args.len() >3 { myiv = args[3].as_str();}
//
//
//     println!("== AES GCM ==");
//     println!("Message: {:?}",msg);
//     println!("Key: {:?}",mykey);
//     println!("IV: {:?}",myiv);
//     println!("Additional: {:?}",myadd);
//
//     let key=&hex_to_bytes( mykey)[..];
//     let iv=&hex_to_bytes( myiv)[..];
//     let plain=msg.as_bytes();
//     let add=myadd.as_bytes();
//
//     let key_size=crypto::aes::KeySize::KeySize128;
//
//     let mut cipher = AesGcm::new(key_size, key, iv, add);
//
//     let mut out: Vec<u8> = repeat(0).take(plain.len()).collect();
//
//
//
//     let mut out_tag: Vec<u8> = repeat(0).take(16).collect();
//
//     cipher.encrypt(plain, &mut out[..],&mut out_tag[..]);
//
//     let mut decipher = AesGcm::new(key_size, key, iv, add);
//     let mut out2: Vec<u8> = repeat(0).take(plain.len()).collect();
//
//     let result = decipher.decrypt(&out[..], &mut out2[..], &out_tag[..]);
//
//     println!("\nEncrypted: {}",hex::encode(out.clone()));
//     if (result==true) { println!("SUCCESSFUL DECRYPTION");}
//     println!("\nDecrypted: {}",str::from_utf8(&out2[..]).unwrap());
// }





