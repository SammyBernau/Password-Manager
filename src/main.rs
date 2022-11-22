

mod encryption;
mod test_encrypt;


// pub use crate::encryption::encrypt_password::final_number;

use crate::test_encrypt::chacha20poly1305_encrypt;

fn main() {
    chacha20poly1305_encrypt("Elegantmuffin4421121@","this is a message");



}









