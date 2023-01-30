use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::ui::createpass::create_pass_ui::create_pass_ui;
use crate::ui::login::login_ui::login_ui;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::json_path::get_json_path;
use gtk::prelude::*;

pub fn check_for_existing_user(application: &adw::Application) {
    if !file_exists(get_json_path()) {
        init_json().expect("JSON init failed");
    }

    let login_window = login_ui(application);

    let manager_account_list_json_struct = get_account_list();
    let stored_pass_len = manager_account_list_json_struct.account_list[0]
        .password
        .len();

    if stored_pass_len == 0 {
        create_pass_ui(application);
    } else {
        login_window.show();
    }
}
