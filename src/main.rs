

mod encryption;
mod test_encrypt;


// pub use crate::encryption::encrypt_password::final_number;
// use openssl::sha::sha256;
use crate::test_encrypt::aes_gcm_example;

fn main() {
    aes_gcm_example();
}




// fn test_hash(){
//     let hash = sha256(b"your data or message");
//     for x in 0..hash.len(){
//         print!("{}",hash[x]);
//     }
// }


//fn add(a: i32,b: i32)-> i32{
//    return a + b;
//}






