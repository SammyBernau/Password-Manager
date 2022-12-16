//Function to create json file during runtime if not already created

//https://blog.devgenius.io/reading-and-writing-a-json-file-in-rust-2731da8d6ad0
//https://betterprogramming.pub/how-to-work-with-json-in-rust-35ddc964009e
//https://docs.rs/json/latest/json/

use std::fs::{File};
use serde_json::json;
use crate::Account;
use crate::json::file_exists::file_exists;
use crate::json::json_structs::{AccountList};
use crate::util_functions::json_path::get_json_path;

pub fn init_json() -> std::io::Result<()>{
    if !file_exists(get_json_path()) {
        File::options().read(true).write(true).create_new(true).open(get_json_path())?;

        //Initialize the json file
        let mut account_list = AccountList::default();

        //Sets up index 0 of account list which will store the users master password
        // Website and username fields will be null
        let master_pass_account = Account {
            website: "master password".to_string(),
            username: "master password".to_string(),
            password: vec![],
            tag: vec![]
        };

        account_list.account_list.push(master_pass_account);
        std::fs::write(get_json_path(), serde_json::to_string(&account_list).unwrap(),)?;
    }
    Ok(())
}
//absolute file path to where exec is run....save json
