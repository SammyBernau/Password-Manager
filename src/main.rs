mod encryption;
mod json;
mod util_functions;
mod ui;



use encryption::string_encrypt::chacha20poly1305_decrypt_string;
use encryption::string_encrypt::chacha20poly1305_encrypt_string;
use crate::ui::build_ui::build_ui;
use crate::json::json_structs::{Account, AccountList};
use gtk::{
    prelude::*
};



const APP_ID: &str = "org.sambernau.password.manager";

fn main() {
    //Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}



