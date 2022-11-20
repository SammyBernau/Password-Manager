//Both examples but second link looks better
//https://asecuritysite.com/symmetric/rust_chacha20poly
//https://medium.com/asecuritysite-when-bob-met-alice/the-near-perfect-encryption-method-and-the-best-programming-lanaguage-meet-aes-gcm-and-rust-33ef68c06677
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




//TODO: Figure out how to use a user's password as key to encryption
//TODO: Figure out why its printing the message in hex and not string!!!!!!!
pub fn test_encrypt_one() -> Result<(), chacha20poly1305::Error>  {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let mut buffer: Vec<u8> = Vec::new(); // Note: buffer needs 16-bytes overhead for auth tag
    buffer.extend_from_slice(b"plaintext message");
    println!("non-encrypted buffer = {buffer:?}");

// Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
    cipher.encrypt_in_place(&nonce, b"", &mut buffer)?;
    println!("encrypted buffer = {:?}", buffer);
// `buffer` now contains the message ciphertext
    assert_ne!(&buffer, b"plaintext message");

// Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    cipher.decrypt_in_place(&nonce, b"", &mut buffer)?;
    println!("back to non-encrypted buffer{:?}", buffer);
    assert_eq!(&buffer, b"plaintext message");
    Ok(())
}



fn hex_to_bytes(s: &str) -> Vec<u8> {
    s.from_hex().unwrap()
}

fn key_hex_to_bytes(s: String) -> Vec<u8> {
    s.from_hex().unwrap()
}

fn pad(mut s: String) -> String{
    while s.len() < 64 {
        s+="0";
    }
    return s;
}


pub fn test_encrypt_two(user_password: &str){
    let mut key = hex::encode(user_password);
    key = pad(key);
    let mut msg="cum";
    let mut myiv="0000000000000000";
    let my_add ="Additional data";
    let args: Vec<String> = env::args().collect();

    if args.len() >1 { msg = args[1].as_str();}
    if args.len() >2 { key = args[2].as_str().parse().unwrap();}
    if args.len() >3 { myiv = args[3].as_str();}
    println!("== ChaCha20/Poly1305 ==");
    println!("Message: {:?}",msg);
    println!("Key: {:?}", key);
    println!("IV: {:?}",myiv);
    println!("Additional data: {:?}", my_add);
    let key=&key_hex_to_bytes(key)[..];
    let iv=&hex_to_bytes( myiv)[..];
    let plain=msg.as_bytes();
    let aad= my_add.as_bytes();

// Encrypting
    let mut c = crypto::chacha20poly1305::ChaCha20Poly1305::new(&key, iv, aad);
    let mut output: Vec<u8> = repeat(0).take(plain.len()).collect();
    let mut outtag: Vec<u8> = repeat(0).take(16).collect();
    c.encrypt(&plain[..], &mut output[..],&mut outtag[..]);
    println!("\nEncrypted: {}",hex::encode(output.clone()));
    println!("\nTag: {}",hex::encode(outtag.clone()));
// Decrypting
    let mut c = crypto::chacha20poly1305::ChaCha20Poly1305::new(&key, iv,aad);
    let mut newoutput: Vec<u8> = repeat(0).take(output.len()).collect();
    c.encrypt(&mut output.clone()[..], &mut newoutput[..],&mut outtag[..]);
    println!("\nDecrypted: {}",str::from_utf8(&newoutput[..]).unwrap());
}





