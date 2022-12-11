mod encryption;
mod json;
mod util_functions;
mod ui;

  use std::hash::Hash;
use std::ops::Deref;
use std::sync::Mutex;
use encryption::encrypt_string::chacha20poly1305_decrypt_string;
use encryption::encrypt_string::chacha20poly1305_encrypt_string;

use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::util_functions::add_account::add_account;
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;
use crate::ui::build_ui::build_ui;
use crate::json::json_structs::{Account, AccountList};
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



const APP_ID: &str = "Password-Manager";

fn main() {
    //Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();

}



