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



