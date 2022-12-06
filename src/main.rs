mod encryption;
mod json;
mod user_functions;

use encryption::encrypt_string::chacha20poly1305_encrypt_string;
use encryption::encrypt_string::chacha20poly1305_decrypt_string;
use crate::json::json_formatter::initialize_json;
use crate::user_functions::add_account::add_account;
use crate::user_functions::remove_account::remove_account;

use crate::json::json_structs::{Account, AccountList};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};



fn main() {
    // let (output, outtag) = chacha20poly1305_encrypt_string("Elegantmuffin4421121@".to_string(), "this is a message".to_string());
    //
    // chacha20poly1305_decrypt_string("Elegantmuffin4421121@".to_string(),output,outtag);
    // //initialize_json().unwrap();

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();


}
const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Sam is the smartest person in the world *middle finger emoji*");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("App")
        .child(&button)
        .show_menubar(true)
        .build();

    // Present window
    window.present();
}










