

mod encryption;
mod test_encrypt;


pub use crate::encryption::encrypt_password::final_number;
use crate::test_encrypt::test_encrypt_one;
// use openssl::sha::sha256;

fn main() {
    //println!("Cum");
    // println!("{}",final_number(2));
    //test_hash();
    test_encrypt_one();
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






