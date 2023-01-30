// use crate::{chacha20poly1305_encrypt_string, hash};
use crate::encryption::hash::string_hash;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::json_path::get_json_path;

pub fn set_master_pass_json(master_pass: String) {
    // let (encrypted_master_pass, encrypted_master_pass_tag) =
    //     chacha20poly1305_encrypt_string(master_pass.clone(), master_pass.clone());
    let hashed_master_pass = string_hash(master_pass);

    let mut account_list = get_account_list();
    account_list.account_list[0].password = hashed_master_pass.clone();
    //account_list.account_list[0].tag = hashed_master_pass_tag.clone();

    std::fs::write(
        get_json_path(),
        serde_json::to_string(&account_list).unwrap(),
    )
    .expect("File save failed");
}
