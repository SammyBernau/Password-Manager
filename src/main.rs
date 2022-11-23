

mod encryption;




use encryption::encrypt_string::chacha20poly1305_encrypt;
use encryption::encrypt_string::chacha20poly1305_decrypt;

fn main() {
    let (output, outtag) = chacha20poly1305_encrypt("Elegantmuffin4421121@", "this is a message");
    chacha20poly1305_decrypt("Elegantmuffin4421121@", output, outtag);




}









