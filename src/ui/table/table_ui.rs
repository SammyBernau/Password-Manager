use crate::aes_string_decryption;
use crate::aes_string_encryption;
use crate::util_functions::json_path::get_json_path;
use crate::util_functions::add_account::add_account;
use crate::ui::table::build_table::build_table;
use crate::util_functions::check_for_account::get_account_list;
use crate::util_functions::remove_account::remove_account;

use crate::ui::util::temp_global_master_pass::get_master_password;
use crate::ui::util::temp_global_master_pass::set_master_password;
use crate::util_functions::set_master_pass_json::set_master_pass_json;

use crate::util_functions::empty_list_check::list_empty;
use gtk::{
    glib::{self, clone},
    prelude::*,
    ApplicationWindow, Button, Grid, Label, ResponseType, ScrolledWindow,
};

pub fn table_ui(application: &adw::Application) -> ApplicationWindow {
    let table_window = ApplicationWindow::new(application);

    table_window.set_title(Some("Sammy's Safe"));
    table_window.set_default_size(800, 300);

    let top_layer_grid = Grid::builder()
        // .column_spacing(5)
        // .row_spacing(10)
        .build();

    table_window.set_child(Some(&top_layer_grid));

    let (tree_view, _list_store) = build_table();

    let sw = ScrolledWindow::builder()
        .child(&tree_view)
        .vexpand(false)
        .vexpand_set(false)
        .max_content_height(10)
        .build();

    top_layer_grid.attach(&sw, 0, 0, 1, 1);

    let list_empty_label = Label::builder()
        .label("List is empty")
        .margin_bottom(5)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    list_empty_label.hide();

    top_layer_grid.attach(&list_empty_label, 0, 0, 1, 1);

    if list_empty() == true {
        list_empty_label.show();
    }

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
            let add_dialog = gtk::Dialog::with_buttons(
            Some("Add Account"),
            Some(&table_window),
            gtk::DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
            );
            add_dialog.set_default_response(ResponseType::Ok);
            let content_area = add_dialog.content_area();

            let website_entry = gtk::Entry::builder()
            .placeholder_text("Enter Website")
            .margin_bottom(5)
            .margin_start(10)
            .margin_end(10)
            .margin_top(5)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();
            website_entry.connect_activate(clone!(@weak add_dialog => move |_| {
            add_dialog.response(ResponseType::Ok);
            }));
            content_area.append(&website_entry);


            let username_entry = gtk::Entry::builder()
            .placeholder_text("Enter Username")
            .margin_bottom(5)
            .margin_start(10)
            .margin_end(10)
            .margin_top(5)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

            username_entry.connect_activate(clone!(@weak add_dialog => move |_| {
            add_dialog.response(ResponseType::Ok);
            }));
            content_area.append(&username_entry);

            let password_entry = gtk::Entry::builder()
            .placeholder_text("Enter Password")
            .margin_bottom(5)
            .margin_start(10)
            .margin_end(10)
            .margin_top(5)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

            password_entry.connect_activate(clone!(@weak add_dialog => move |_| {
            add_dialog.response(ResponseType::Ok);
            }));
            content_area.append(&password_entry);

            let list_store_copy = _list_store.clone();
            add_dialog.connect_response(clone!(@weak website_entry, @weak username_entry, @weak password_entry => move |add_dialog, resp| {
                let website_text = website_entry.text();
                let username_text = username_entry.text();
                let password_text = password_entry.text();
                if (!website_text.is_empty() && !username_text.is_empty() && !password_text.is_empty()) && resp == ResponseType::Ok {

                    //Updates table view with new entry
                    list_store_copy.insert_with_values(
                        None,
                        &[
                            (0, &website_text),
                            (1, &username_text),
                            (2, &password_text),
                        ],
                    );

                    //Updates json and saves new entry right away
                    add_account(get_master_password(),
                        website_text.parse().unwrap(),
                        username_text.parse().unwrap(),
                        password_text.parse().unwrap()
                    //Add better error handling here
                    ).expect("Add account failed");
                }
                add_dialog.close();
            }));
            add_dialog.show()
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
            clone!(@weak table_window, @weak _list_store, @weak tree_view, @weak list_empty_label => move |_| {
            let delete_dialog = gtk::Dialog::with_buttons(
                Some("Delete Row"),
                Some(&table_window),
                gtk::DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
            );

                let content_area = delete_dialog.content_area();
                let delete_label = Label::builder()
                .label("Are you sure you want to delete?")
                .margin_bottom(10)
                .margin_start(10)
                .margin_end(10)
                .margin_top(10)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .build();
                content_area.append(&delete_label);

                delete_dialog.set_default_size(200,50);
                delete_dialog.set_default_response(ResponseType::Ok);
                delete_dialog.can_focus();

                let list_store_copy = _list_store.clone();
                delete_dialog.connect_response(move |delete_dialog, resp| {
                    let selection = tree_view.selection();
                    if let Some((model,selected_row)) = selection.selected() { //Gets the user selected row
                        if resp == ResponseType::Ok {
                            let website: String = model.get_value(&selected_row, 0).get().unwrap();
                            list_store_copy.remove(&selected_row); //Removes it if response is OK
                            remove_account(website.parse().unwrap()).expect("Remove account failed"); //TODO Better error handling here
                        }
                        delete_dialog.close();

                    }});
                delete_dialog.show()

            }));

    // let save_button = Button::builder()
    //     .label("Save")
    //     .margin_bottom(10)
    //     .margin_start(10)
    //     .margin_end(10)
    //     .margin_top(10)
    //     .valign(gtk::Align::Center)
    //     .halign(gtk::Align::Center)
    //     .build();

    // save_button.connect_clicked(
    //     clone!(@weak table_window, @weak _list_store, @weak tree_view => move |_| {
    //         let master_pass_string = get_master_password();
    //         init_json();
    //         set_master_pass_json(master_pass_string);
    //
    //     let mut manager_account_list_json_struct = get_account_list();
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
    //         save_dialog.show()
    // }), );

    let reset_pass_button = Button::builder()
        .label("Reset Master Password")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    reset_pass_button.connect_clicked(clone! (@weak table_window, @weak application => move |_| {
            let reset_pass_dialog = gtk::Dialog::with_buttons(
            Some("Reset Password"),
            Some(&table_window),
            gtk::DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
            );
            reset_pass_dialog.set_default_response(ResponseType::Ok);

            reset_pass_dialog.set_default_size(200, 100);
            let content_area = reset_pass_dialog.content_area();

            let reset_label = Label::builder()
                .label("Please enter a new password\n \t DON'T LOSE IT!")
                .margin_bottom(5)
                .margin_start(10)
                .margin_end(10)
                .margin_top(10)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .build();
            content_area.append(&reset_label);


            //user types in new password
            let password_entry = gtk::Entry::builder()
            .placeholder_text("New Password")
            .margin_bottom(5)
            .margin_start(10)
            .margin_end(10)
            .margin_top(5)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

            password_entry.connect_activate(clone!(@weak reset_pass_dialog => move |_| {
            reset_pass_dialog.response(ResponseType::Ok);
            }));
            content_area.append(&password_entry);

            //second box where user types in new password again
            let password_entry_compare = gtk::Entry::builder()
            .placeholder_text("Re-enter Password")
            .margin_bottom(5)
            .margin_start(10)
            .margin_end(10)
            .margin_top(5)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();

            password_entry_compare.connect_activate(clone!(@weak reset_pass_dialog => move |_| {
            reset_pass_dialog.response(ResponseType::Ok);
            }));
            content_area.append(&password_entry_compare);

            reset_pass_dialog.connect_response(clone!(@weak password_entry, @weak password_entry_compare, @weak table_window, @weak _list_store, @weak reset_label => move |reset_pass_dialog, resp| {

                let check_password_entry: String = password_entry.text().parse().unwrap();
                let check_password_entry_compare: String = password_entry_compare.text().parse().unwrap();
                if (password_entry.text() == password_entry_compare.text()) && resp == ResponseType::Ok && (check_password_entry.len() != 0 && check_password_entry_compare.len() != 0) {

                    let new_password: String = password_entry.text().parse().unwrap();

                    //Set new master password
                    set_master_pass_json(new_password.clone());

                    //Get master account list
                    let mut manager_account_list_json_struct = get_account_list();

                    for account in manager_account_list_json_struct.account_list.iter_mut() {
                        if account.website !="master password" && account.username !="master password" {
                            //decrypt each account pass with old password first
                            let old_pass = account.password.clone();
                            let old_nonce = account.nonce.clone();
                            let old_master_pass = get_master_password();

                            let decrypted_pass_old = aes_string_decryption(
                                old_master_pass,
                                old_pass,
                                old_nonce);

                            //encrypt each account pass with new password and re-insert
                            let (encrypted_pass_new, encrypted_nonce_new) = aes_string_encryption(new_password.clone(), String::from_utf8(decrypted_pass_old).expect("Found invalid UTF-8"));
                            account.password = encrypted_pass_new;
                            account.nonce = encrypted_nonce_new;
                        }
                    }
                    //save all changes to json
                        std::fs::write(get_json_path(), serde_json::to_string(&manager_account_list_json_struct.clone()).unwrap()).expect("JSON failed to load"); //TODO Better error handling
                        //Save string value of new master password (lasts until app exited)
                        set_master_password(password_entry.clone().text().parse().unwrap());
                        reset_pass_dialog.close();
                    } else {
                    reset_label.set_text("New password length must greater than zero");
                }

                if resp == ResponseType::Cancel {
                    reset_pass_dialog.close();
                }

            }));
            reset_pass_dialog.show()
        }));

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
    button_grid.attach(&reset_pass_button, 2, 0, 1, 1);
    top_layer_grid.attach(&button_grid, 0, 1, 1, 1);

    return table_window;
}
