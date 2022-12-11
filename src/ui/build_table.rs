
use crate::chacha20poly1305_encrypt_string;
use crate::chacha20poly1305_decrypt_string;

use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::util_functions::add_account::add_account;
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;
use crate::ui::temp_global_master_pass::get_master_password;
use core::str;
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



pub fn build_table() -> (TreeView, ListStore) {
    let list_store = ListStore::new(&[glib::Type::STRING].repeat(4));
    let tree_view = TreeView::builder()
        .hexpand(true)
        .vexpand(true)
        .enable_grid_lines(gtk::TreeViewGridLines::Both)
        .model(&list_store)
        .build();

    for (i, title) in ["Website", "Username", "Password"].iter().enumerate() {
        let renderer = CellRendererText::builder()
            .editable(true)
            .xalign(0.0)
            .single_paragraph_mode(true)
            .build();

        tree_view.insert_column_with_attributes(
            i.try_into().unwrap(),
            title,
            &renderer,
            &[(&"text", i.try_into().unwrap())],
        );

        let list_store_copy = list_store.clone();
        renderer.connect_edited(move |_renderer, row, text| {
            list_store_copy.set_value(
                &list_store_copy.iter(&row).unwrap(),
                i.try_into().unwrap(),
                &text.to_value(),
            );
        });
    }

    if file_exists("password_manager_json.json") {
        let manager_account_list_json_struct = get_account_list();
        let mut manager_account_list = manager_account_list_json_struct.account_list;

            for account in manager_account_list.iter_mut() {
                if account.website != "null" && account.username != "null" {
                    let encrypted_account_pass = account.password.clone();
                    let encrypted_account_tag = account.tag.clone();
                    let decrypted_pass = chacha20poly1305_decrypt_string(
                        get_master_password(),
                        encrypted_account_pass,
                        encrypted_account_tag);


                    list_store.insert_with_values(
                        None,
                        &[
                            (0, &account.website),
                            (1, &account.username),
                            (2, &str::from_utf8(&decrypted_pass[..]).unwrap())
                        ],
                    );
                }
            }
    }
    (tree_view, list_store)
}