use crate::json::json_structs::{Account, AccountList};
use crate::chacha20poly1305_encrypt_string;
use crate::user_functions::check_for_account::check_account_existence;




pub fn add_account(master_password: String, website: String, username: String, account_password: String) -> std::io::Result<()>{
    if check_account_existence(&website) == false{
        let json_file = "password_manager_json.json";
        let mut entire_account_list = {
            let json_output = std::fs::read_to_string(json_file)?;

            //Loads the AccountList structure from the string
            serde_json::from_str::<AccountList>(&json_output).unwrap()
        };

        let (encrypted_account_password, encrypted_account_password_tag) = chacha20poly1305_encrypt_string(master_password, account_password);
        let new_account = Account {
            website,
            username,
            password: encrypted_account_password,
            tag: encrypted_account_password_tag
        };


        //Adds the new account to the json formatted array
        entire_account_list.account_list.push(new_account);

        std::fs::write(json_file, serde_json::to_string(&entire_account_list).unwrap(), )?;
    }
    Ok(())

}