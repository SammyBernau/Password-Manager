use crate::ui::login::login_ui::login_ui;
use crate::util_functions::set_master_pass_json::set_master_pass_json;
use gtk::{
    glib::{self, clone},
    prelude::*,
    ApplicationWindow, Button, Entry, Grid, Label,
};

pub fn create_pass_ui(application: &adw::Application) {
    let create_pass_window = ApplicationWindow::new(application);
    create_pass_window.set_title(Some("Sammy's Safe Create Password"));
    create_pass_window.set_default_size(400, 150);

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

    let create_pass_entry_compare = Entry::builder()
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    create_pass_entry_compare.set_placeholder_text(Some("Re-enter password"));

    // create_pass_entry.set_invisible_char(Some('*'));

    password_entry_grid.attach(&create_pass_entry, 0, 1, 1, 1);
    password_entry_grid.attach(&create_pass_entry_compare, 0, 2, 1, 1);

    let ok_button = Button::builder()
        .label("Ok")
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    let error_label = Label::builder()
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();

    error_label.set_text("Please enter a password");

    top_layer_grid.attach(&error_label, 1, 0, 1, 1);
    top_layer_grid.attach(&password_entry_grid, 1, 1, 1, 1);
    top_layer_grid.attach(&ok_button, 1, 2, 1, 1);

    ok_button.connect_clicked(clone!(
        @weak create_pass_window,
        @weak create_pass_entry,
        @weak create_pass_entry_compare,
        @weak error_label,
        @weak application => move |_|{

            //Check that both entries of user password match
            if create_pass_entry.text() == create_pass_entry_compare.text() {
                let create_pass_text = create_pass_entry.text();

                //Check that the user actually entered a password
                if !create_pass_text.is_empty() {
                    let login_window = login_ui(&application);

                    set_master_pass_json(create_pass_text.parse().unwrap());

                    create_pass_window.close();
                    login_window.show();
                } else {
                    error_label.set_text("Password can't be empty");
                    error_label.show();
                }
            } else {
                error_label.set_text("Passwords don't match");
                error_label.show();


            }
    }));
    create_pass_window.show();
}
