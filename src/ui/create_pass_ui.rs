

use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::util_functions::add_account::add_account;
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;
use crate::ui::login_ui::login_ui;
use crate::json::json_structs::{Account, AccountList};
use crate::chacha20poly1305_encrypt_string;

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



pub fn create_pass_ui(application: &Application) {

    let create_pass_window = ApplicationWindow::new(application);
    create_pass_window.set_title(Some("Please create a master password "));
    create_pass_window.set_default_size(400,150);

    let top_layer_grid = Grid::builder()
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_start(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    create_pass_window.set_child(Some(&top_layer_grid));

    let password_entry_grid = Grid::builder()
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    let create_pass_entry = Entry::builder()
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    create_pass_entry.set_placeholder_text(Some("Password"));
    create_pass_entry.set_invisible_char(Some('*'));

    password_entry_grid.attach(&create_pass_entry, 0, 1, 1,1 );


    let ok_button = Button::builder()
        .label("Ok")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();


    top_layer_grid.attach(&password_entry_grid, 1, 1, 1, 1);
    top_layer_grid.attach(&ok_button, 1, 2, 1, 1);



    ok_button.connect_clicked(clone!(@weak create_pass_window, @weak create_pass_entry,@weak application => move |_|{
        let create_pass_text = create_pass_entry.text();
        if !create_pass_text.is_empty() {
            let login_window = login_ui(&application);
            let (encrypted_master_pass, encrypted_master_pass_tag) = chacha20poly1305_encrypt_string(
                create_pass_text.parse().unwrap(),
                create_pass_text.parse().unwrap(),);
            let mut manager_account_list_json_struct = get_account_list();
            manager_account_list_json_struct.account_list[0].password = encrypted_master_pass.clone();
            manager_account_list_json_struct.account_list[0].tag = encrypted_master_pass_tag.clone();
            std::fs::write("password_manager_json.json", serde_json::to_string(&manager_account_list_json_struct).unwrap());
            create_pass_window.close();
            login_window.show();

        } else {
                let invalid_pass_dialog = gtk::Dialog::with_buttons(
                Some("Enter Valid Password"),
                Some(&create_pass_window),
                gtk::DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok)],
                );
                invalid_pass_dialog.set_default_response(ResponseType::Ok);
                invalid_pass_dialog.can_focus();
                invalid_pass_dialog.connect_response(move |invalid_pass_dialog, resp| {
                    if resp == ResponseType::Ok {
                    invalid_pass_dialog.close();
                    }
                });
                invalid_pass_dialog.show();
            }
    }));
    create_pass_window.show();
}