use crate::chacha20poly1305_encrypt_string;

use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::util_functions::add_account::add_account;
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;
use crate::ui::login_ui::login_ui;
use crate::json::json_structs::{Account, AccountList};
use crate::ui::create_pass_ui::create_pass_ui;

use gtk::glib::boxed::Boxed;
use gtk::AccessibleProperty::Orientation;
use gtk::CellRendererAccelMode::Gtk;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
    Application, ApplicationWindow, Button, CellRendererText, Entry, Grid, Label, ListBox,
    ListStore, ListView, ResponseType, ScrolledWindow, TreeIter, TreePath, TreeView,
};



pub fn check_for_existing_user(application: &Application) {
    if !file_exists("password_manager_json.json") {
        init_json().expect("JSON init failed");
    }

    let login_window = login_ui(application);

    let mut manager_account_list_json_struct = get_account_list();
    let stored_pass_len = manager_account_list_json_struct.account_list[0].password.len();

    if stored_pass_len == 0 {
        create_pass_ui(application);
    } else {
        login_window.show();
    }
}