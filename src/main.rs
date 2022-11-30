mod encryption;
mod json;
mod user_functions;

use encryption::encrypt_string::chacha20poly1305_encrypt_string;
use encryption::encrypt_string::chacha20poly1305_decrypt_string;
use crate::json::json_formatter::initialize_json;
use crate::user_functions::add_account::add_account;
use crate::user_functions::remove_account::remove_account;

use crate::json::json_structs::{Account, AccountList};



fn main() {
    // let (output, outtag) = chacha20poly1305_encrypt_string("Elegantmuffin4421121@".to_string(), "this is a message".to_string());
    //
    // chacha20poly1305_decrypt_string("Elegantmuffin4421121@".to_string(),output,outtag);
    // //initialize_json().unwrap();
    // add_account("Elegantmuffin4421121@".to_string(),
    //             "Pornhub".to_string(),
    //             "sbernau".to_string(),
    //             "1121442@".to_string()).expect("TODO: panic message");

    remove_account("Pornhub".to_string());

}










