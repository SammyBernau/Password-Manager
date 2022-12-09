mod encryption;
mod json;
mod user_functions;

use std::ptr::{null, null_mut};
use encryption::encrypt_string::chacha20poly1305_encrypt_string;
use encryption::encrypt_string::chacha20poly1305_decrypt_string;
use crate::json::json_formatter::initialize_json;
use crate::user_functions::add_account::add_account;
use crate::user_functions::remove_account::remove_account;

use crate::json::json_structs::{Account, AccountList};

use gtk::{glib::{self, clone}, prelude::*, ResponseType, Application, ApplicationWindow, Button, CellRendererText, Entry, gio, Grid, Label, ListBox, ListStore, ListView, ScrolledWindow, TreeView, TreeIter, TreePath};
use gtk::AccessibleProperty::Orientation;
use gtk::CellRendererAccelMode::Gtk;
use gtk::glib::boxed::Boxed;


const APP_ID: &str = "Password-Manager";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_table_ui);

    // Run the application
    app.run();
}

fn table_ui() -> (TreeView, ListStore) {
    let list_store = ListStore::new(&[glib::Type::STRING].repeat(4));
    let tree_view = TreeView::builder()
        .hexpand(true)
        .vexpand(true)
        .enable_grid_lines(gtk::TreeViewGridLines::Both)
        .model(&list_store)
        .build();


    for (i, title) in ["Website", "Username", "Password"]
        .iter()
        .enumerate()
    {
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
                &text.to_value()
            );
        });
    }


    list_store.insert_with_values(
        None,
        &[
            (0, &"Test1")
        ],
    );
    (tree_view, list_store)
}

fn build_table_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title(Some("Sammy's Safe"));
    window.set_default_size(800, 300);

    let button_grid_holder = Grid::builder()
        // .column_spacing(5)
        // .row_spacing(10)
        .build();

    window.set_child(Some(&button_grid_holder));

    let (tree_view, _list_store) = table_ui();
    let sw = ScrolledWindow::builder()
        .child(&tree_view)
        .vexpand(false)
        .vexpand_set(false)
        .max_content_height(10)
        .build();

    // column, row, width, height
    button_grid_holder.attach(&sw, 0, 0, 1, 1);

    let add_button = gtk::Button::builder()
        .label("Add")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .build();
    add_button.connect_clicked(clone!(@weak window, @weak _list_store => move |_| {
        let dialog = gtk::Dialog::with_buttons(
            Some("Add Account"),
            Some(&window),
            gtk::DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        dialog.set_default_response(ResponseType::Ok);
        let content_area = dialog.content_area();
        let list_store_copy = _list_store.clone();
        let website_entry = gtk::Entry::new();
        website_entry.insert_text(&"Enter Website", &mut 0);
        website_entry.connect_activate(clone!(@weak dialog => move |_| {
            dialog.response(ResponseType::Ok);
        }));
        content_area.append(&website_entry);

        let username_entry = gtk::Entry::new();
        username_entry.insert_text(&"Enter Username", &mut 0);
        username_entry.connect_activate(clone!(@weak dialog => move |_| {
            dialog.response(ResponseType::Ok);
        }));
        content_area.append(&username_entry);

        let password_entry = gtk::Entry::new();
        password_entry.insert_text(&"Enter Password", &mut 0);
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
            }
            dialog.close();
        }));

        dialog.show()
    }));


    let delete_button = gtk::Button::builder()
        .label("Delete")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .build();
    delete_button.connect_clicked(clone!(@weak window, @weak _list_store, @weak tree_view => move |_| {
        let dialog = gtk::Dialog::with_buttons(
            Some("Delete Row"),
            Some(&window),
            gtk::DialogFlags::MODAL,
            &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );
        dialog.set_default_response(ResponseType::Ok);
        dialog.can_focus();
        let content_area = dialog.content_area();


        let list_store_copy = _list_store.clone();
        dialog.connect_response(move |dialog, resp| {
            let selection = tree_view.selection();
        if let Some((s,selected_row)) = selection.selected() {
                if resp == ResponseType::Ok {
                list_store_copy.remove(&selected_row);
            }
            dialog.close();
        }});
        dialog.show()
    }));

    let add_delete_grid = gtk::Grid::builder()
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_start(10)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
    //add_delete_grid.set_column_homogeneous(true);
    add_delete_grid.attach(&add_button, 0, 0, 1, 1);
    add_delete_grid.attach(&delete_button, 1, 0, 1, 1);
    // button_grid_holder.insert_column(5);
    button_grid_holder.attach(&add_delete_grid, 0, 1, 1, 1);
    window.show();
}








