//Function to create json file during runtime if not already created

//https://blog.devgenius.io/reading-and-writing-a-json-file-in-rust-2731da8d6ad0
//https://betterprogramming.pub/how-to-work-with-json-in-rust-35ddc964009e
//https://docs.rs/json/latest/json/

use std::fs::{File};
use std::path::Path;

use crate::json::json_structs::{AccountList};


pub fn initialize_json()-> std::io::Result<()>{
    let password_manager_json = "password_manager_json.json";

    if !file_exists(password_manager_json) {
        File::options().read(true).write(true).create_new(true).open(password_manager_json)?;
        println!("Created Json");


        //Initialize the json file
        let account_list = AccountList::default();
        std::fs::write(password_manager_json, serde_json::to_string(&account_list).unwrap(),)?;
    }
    Ok(())
}

fn file_exists(file_path: &str) -> bool {
    let from_string = Path::new(&file_path);

    let mut existence: bool = true;
    existence = Path::new(from_string).exists();

    if existence == true {
        true
    } else {
        false
    }
}