use crate::util_functions::check_for_account::get_account_list;

pub fn list_empty() -> bool {
    let manager_account_list_json_struct = get_account_list();
    let manager_account_list = manager_account_list_json_struct.account_list;

    manager_account_list.is_empty()
}
