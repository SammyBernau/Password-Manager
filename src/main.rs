mod encryption;
mod json;
mod ui;
mod util_functions;
use crate::json::json_structs::{Account, AccountList};
use crate::ui::build_ui::build_ui;
use crate::encryption::aes_encryption::{aes_string_decryption, aes_string_encryption};
use gtk::prelude::*;

const APP_ID: &str = "org.sambernau.password.manager";

fn main() {
    //Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();

}
