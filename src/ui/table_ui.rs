
use crate::chacha20poly1305_encrypt_string;
use crate::json::file_exists::file_exists;
use crate::json::init_json::init_json;
use crate::util_functions::add_account::add_account;
use crate::util_functions::check_for_account::check_account_existence;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;
use crate::ui::build_table::build_table;
use crate::ui::temp_global_master_pass::get_master_password;
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

pub fn table_ui(application: &Application) -> ApplicationWindow {
        let table_window = ApplicationWindow::new(application);

        table_window.set_title(Some("Sammy's Safe"));
        table_window.set_default_size(800, 300);

        let button_grid_holder = Grid::builder()
            // .column_spacing(5)
            // .row_spacing(10)
            .build();

        table_window.set_child(Some(&button_grid_holder));


        let (tree_view, _list_store) = build_table();


        let sw = ScrolledWindow::builder()
            .child(&tree_view)
            .vexpand(false)
            .vexpand_set(false)
            .max_content_height(10)
            .build();


        button_grid_holder.attach(&sw, 0, 0, 1, 1);

        let add_button = Button::builder()
            .label("Add")
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();
        add_button.connect_clicked(clone!(@weak table_window, @weak _list_store => move |_| {
        let dialog = gtk::Dialog::with_buttons(
            Some("Add Account"),
            Some(&table_window),
            gtk::DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        dialog.set_default_response(ResponseType::Ok);
        let content_area = dialog.content_area();
        let list_store_copy = _list_store.clone();


        let website_entry = gtk::Entry::new();
        website_entry.set_placeholder_text(Some("Enter Website"));
        website_entry.connect_activate(clone!(@weak dialog => move |_| {
            dialog.response(ResponseType::Ok);
        }));
        content_area.append(&website_entry);

        let username_entry = gtk::Entry::new();
        username_entry.set_placeholder_text(Some("Enter Username"));
        username_entry.connect_activate(clone!(@weak dialog => move |_| {
            dialog.response(ResponseType::Ok);
        }));
        content_area.append(&username_entry);

        let password_entry = gtk::Entry::new();
        password_entry.set_placeholder_text(Some("Enter Password"));
        password_entry.connect_activate(clone!(@weak dialog => move |_| {
            dialog.response(ResponseType::Ok);
        }));
        content_area.append(&password_entry);

        let list_store_copy = _list_store.clone();
        dialog.connect_response(clone!(@weak website_entry, @weak username_entry, @weak password_entry => move |dialog, resp| {
            let website_text = website_entry.text();
            let username_text = username_entry.text();
            let password_text = password_entry.text();
            if (!website_text.is_empty() && !username_text.is_empty() && !password_text.is_empty()) && resp == ResponseType::Ok {
                list_store_copy.insert_with_values(
                        None,
                    &[
                        (0, &website_text),
                        (1, &username_text),
                        (2, &password_text),
                    ],
                );
                    add_account(get_master_password(),
                        website_text.parse().unwrap(),
                        username_text.parse().unwrap(),
                        password_text.parse().unwrap()
                    );
            }
            dialog.close();
        }));

        dialog.show()
    }));

        let delete_button = Button::builder()
            .label("Delete")
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

        delete_button.connect_clicked(
            clone!(@weak table_window, @weak _list_store, @weak tree_view => move |_| {
            let dialog = gtk::Dialog::with_buttons(
                Some("Delete Row"),
                Some(&table_window),
                gtk::DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
            );
            dialog.set_default_response(ResponseType::Ok);
            dialog.can_focus();

            let list_store_copy = _list_store.clone();
            dialog.connect_response(move |dialog, resp| {
                let selection = tree_view.selection();
            if let Some((model,selected_row)) = selection.selected() {
                    if resp == ResponseType::Ok {
                            let website: String = model.get_value(&selected_row, 0).get().unwrap();
                            list_store_copy.remove(&selected_row);
                            remove_account(website.parse().unwrap());
                }
                dialog.close();
            }});
            dialog.show()
        }),
        );

        let save_button = Button::builder()
            .label("Save")
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .margin_top(10)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();


        // save_button.connect_clicked(
        //     clone!(@weak table_window, @weak _list_store, @weak tree_view => move |_| {
        //     init_json();
        //     let (encrypted_master_pass, encrypted_master_pass_tag) = chacha20poly1305_encrypt_string(
        //         get_master_password(),
        //         get_master_password(),);
        //
        //     let mut manager_account_list_json_struct = get_account_list();
        //     manager_account_list_json_struct.account_list[0].password = encrypted_master_pass.clone();
        //     manager_account_list_json_struct.account_list[0].tag = encrypted_master_pass_tag.clone();
        //
        //     // Create a TreeIter for the first row
        //     //let mut iter = store.iter_first().unwrap();
        //     // Iterate through the ListStore and get the values of each row
        //     _list_store.foreach(|model, path, iter| {
        //         let website: String = model.get_value(iter,0).get().unwrap();
        //         let username: String = model.get_value(iter,1).get().unwrap();
        //         let password: String = model.get_value(iter,2).get().unwrap();
        //
        //         add_account(get_master_password(),
        //             website.parse().unwrap(),
        //             username.parse().unwrap(),
        //             password.parse().unwrap());
        //
        //         true
        //     });
        //
        //     let save_dialog = gtk::Dialog::with_buttons(
        //         Some("Save Successful"),
        //         Some(&table_window),
        //         gtk::DialogFlags::MODAL,
        //         &[("Ok", ResponseType::Ok)],
        //     );
        //
        //     save_dialog.set_default_response(ResponseType::Ok);
        //     save_dialog.can_focus();
        //
        //     save_dialog.connect_response(move |save_dialog, resp| {
        //             if resp == ResponseType::Ok {
        //             save_dialog.close();
        //         }
        //     });
        //
        // }),
        // );


        let button_grid = Grid::builder()
            .margin_bottom(10)
            .margin_end(10)
            .margin_start(10)
            .margin_start(10)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .row_spacing(6)
            .column_spacing(6)
            .build();

        button_grid.attach(&add_button, 0, 0, 1, 1);
        button_grid.attach(&delete_button, 1, 0, 1, 1);
        // button_grid.attach(&save_button, 2, 0, 1, 1);
        button_grid_holder.attach(&button_grid, 0, 1, 1, 1);


    return table_window;
}