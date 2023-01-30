use crate::aes_string_encryption;
use crate::json::json_structs::{Account, AccountList};
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::json_path::get_json_path;



pub fn add_account(
    master_password: String,
    website: String,
    username: String,
    account_password: String,
) -> std::io::Result<()>  {
    //TODO handle result correctly

    if check_account_existence(&website) == false {
        //let json_file = "password_manager_json.json";
        let mut entire_account_list = {
            let json_output = std::fs::read_to_string(get_json_path())?;

            //Loads the AccountList structure from the string
            serde_json::from_str::<AccountList>(&json_output).unwrap()
        };

        let (encrypted_account_password, encrypted_account_password_nonce) =
            aes_string_encryption(master_password, account_password);
        let new_account = Account {
            website,
            username,
            password: encrypted_account_password,
            nonce: encrypted_account_password_nonce,
        };

        //Adds the new account to the json formatted array
        entire_account_list.account_list.push(new_account);

        std::fs::write(
            get_json_path(),
            serde_json::to_string(&entire_account_list).unwrap(),
        )?;
    }
    Ok(())
}
