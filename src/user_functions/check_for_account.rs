use std::fs::File;
use std::io::Read;
use crate::AccountList;



fn get_account_list() -> AccountList {
    let mut json_file = File::open("password_manager_json.json").unwrap();

    let entire_account_list = {
        let mut json_data = String::new();
        json_file.read_to_string(&mut json_data).unwrap();

        serde_json::from_str::<AccountList>(&json_data).unwrap()
    };

    entire_account_list

}

pub fn check_account_existence(check_website: &String) -> bool {
   let entire_account_list= get_account_list();

    let mut existence = true;
    for index in 0..entire_account_list.account_list.len() {
        if entire_account_list.account_list[index].website.eq(check_website){
            existence = true;
        } else {
           existence = false;
        }
    }
   existence
}