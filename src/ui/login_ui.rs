use crate::chacha20poly1305_encrypt_string;

use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::util_functions::add_account::add_account;
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;
use crate::ui::table_ui::table_ui;
use crate::ui::temp_global_master_pass::MASTER_PASSWORD;
use crate::ui::temp_global_master_pass::set_master_password;



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



pub fn login_ui(application: &Application) -> ApplicationWindow {
    let login_window = ApplicationWindow::new(application);
    login_window.set_title(Some("Sammy's Safe Login"));
    login_window.set_default_size(400, 150);

    let login_top_layer_grid = Grid::builder()
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_start(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
    login_window.set_child(Some(&login_top_layer_grid));

    let password_entry_grid = Grid::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let password_entry = Entry::builder()
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    password_entry.set_placeholder_text(Some("Password"));
    password_entry.set_invisible_char(Some('*'));
    password_entry_grid.attach(&password_entry, 0, 1, 1, 1);
    login_top_layer_grid.attach(&password_entry_grid, 1, 1, 1, 1);

    let login_button = Button::builder()
        .label("Login")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();




    login_button.connect_clicked(clone!(@weak login_window, @weak application, @weak password_entry => move |_|{
        let password_text = password_entry.text();
        if !password_text.is_empty() {
            let password_text_clone = password_text.clone();
            set_master_password(password_text_clone.parse().unwrap());
            let table_window = table_ui(&application);
            //encrypt user entered pass for comparison to stored pass
            let (user_entered_encrypted_master_pass, user_entered_encrypted_master_pass_tag) = chacha20poly1305_encrypt_string(
                password_text.parse().unwrap(),
                password_text.parse().unwrap());
            let mut manager_account_list_json_struct = get_account_list();
            let stored_master_pass = manager_account_list_json_struct.account_list[0].password.clone();
            let stored_master_pass_tag = manager_account_list_json_struct.account_list[0].tag.clone();

            if ((user_entered_encrypted_master_pass == stored_master_pass)) {
                login_window.close();
                table_window.show();
            }
        }
    }));

    let login_window_cancel_button = Button::builder()
        .label("Cancel")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .build();

    login_window_cancel_button.connect_clicked(clone!(@weak login_window => move |_|{
        login_window.close();
    }));

    let login_cancel_grid = Grid::builder()
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_start(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    //add reset password to login_ui
    login_cancel_grid.attach(&login_button.clone(), 0, 0, 1, 1);
    login_cancel_grid.attach(&login_window_cancel_button, 1, 0, 1, 1);
    login_top_layer_grid.attach(&login_cancel_grid, 1, 2, 1, 1);

    return login_window;
}