
//TODO Testing





//TODO OPTION 1

// extern crate gio;
// extern crate gtk;
//
// use gio::prelude::*;
// use gtk::prelude::*;
//
// use std::env::args;
// use std::rc::Rc;
// use gtk::true_;
//
//
// fn main() {
//     let application = gtk::Application::new(
//         Some("com.bernau.password-manager"),
//         Default::default(),
//     )
//         .expect("Initialization failed...");
//
//     application.connect_startup(|app| {
//         build_ui(app);
//     });
//
//     application.connect_activate(|_| {});
//
//     application.run(&args().collect::<Vec<_>>());
// }
//
//
//
// #[derive(Debug)]
// #[repr(i32)]
// enum Columns {
//     Website,
//     Username,
//     Password
// }
//
// fn build_ui(application: &gtk::Application) {
//     let window = gtk::ApplicationWindow::new(application);
//
//     window.set_title("List Store");
//     window.set_border_width(10);
//     window.set_position(gtk::WindowPosition::Center);
//     window.set_default_size(800, 600);
//
//     let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
//     window.add(&vbox);
//
//     let label = gtk::Label::new(Some(
//         "Welcome to Sammy's Safe",
//     ));
//     vbox.add(&label);
//
//     let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
//     sw.set_shadow_type(gtk::ShadowType::EtchedIn);
//     sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
//     vbox.add(&sw);
//
//     let model = Rc::new(create_model());
//     let treeview = gtk::TreeView::with_model(&*model);
//     treeview.set_vexpand(true);
//     treeview.set_search_column(Columns::Website as i32);
//
//     sw.add(&treeview);
//
//     add_columns(&model, &treeview);
//
//     window.show_all();
//
//     // let model = model.clone();
//     // timeout_add(80, move || spinner_timeout(&model));
// }
//
// struct Data {
//     website: String,
//     username: String,
//     password: String
// }
//
// fn create_model() -> gtk::ListStore {
//     let col_types: [glib::Type; 3] = [
//         glib::Type::String,
//         glib::Type::String,
//         glib::Type::String,
//     ];
//
//     let data: [Data; 2] = [
//         Data {
//             website: "Steam".to_string(),
//             username: "Sbernau".to_string(),
//             password: "1234".to_string(),
//
//         },
//         Data {
//             website: "Spotify".to_string(),
//             username: "Bernaus".to_string(),
//             password: "5678".to_string(),
//         }
//     ];
//
//     let store = gtk::ListStore::new(&col_types);
//
//     let col_indices: [u32; 3] = [0, 1, 2];
//
//     for (d_idx, d) in data.iter().enumerate() {
//         let icon_name = if d_idx == 1 || d_idx == 3 {
//             "battery-caution-charging-symbolic"
//         } else {
//             ""
//         };
//
//         let sensitive = d_idx != 3;
//
//         let values: [&dyn ToValue; 3] = [
//             &d.website,
//             &d.username,
//             &d.password,
//         ];
//         store.set(&store.append(), &col_indices, &values);
//     }
//
//     store
// }
//
// // fn fixed_toggled<W: IsA<gtk::CellRendererToggle>>(
// //     model: &gtk::ListStore,
// //     _w: &W,
// //     path: gtk::TreePath,
// // ) {
// //     let iter = model.get_iter(&path).unwrap();
// //     let mut fixed = model
// //         .get_value(&iter, Columns::Fixed as i32)
// //         .get_some::<bool>()
// //         .unwrap_or_else(|err| {
// //             panic!(
// //                 "ListStore value for {:?} at path {}: {}",
// //                 Columns::Fixed,
// //                 path,
// //                 err
// //             )
// //         });
// //     fixed = !fixed;
// //     model.set_value(&iter, Columns::Fixed as u32, &fixed.to_value());
// // }
//
// fn add_columns(model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {
//     // Column for fixed toggles
//     // {
//     //     let renderer = gtk::CellRendererToggle::new();
//     //     let model_clone = model.clone();
//     //     renderer.connect_toggled(move |w, path| fixed_toggled(&model_clone, w, path));
//     //     let column = gtk::TreeViewColumn::new();
//     //     column.pack_start(&renderer, true);
//     //     column.set_title("Fixed?");
//     //     column.add_attribute(&renderer, "active", Columns::Fixed as i32);
//     //     column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
//     //     column.set_fixed_width(50);
//     //     treeview.append_column(&column);
//     // }
//
//     // Column for bug numbers
//     // {
//     //     let renderer = gtk::CellRendererText::new();
//     //     let column = gtk::TreeViewColumn::new();
//     //     column.pack_start(&renderer, true);
//     //     column.set_title("Bug number");
//     //     column.add_attribute(&renderer, "text", Columns::Number as i32);
//     //     column.set_sort_column_id(Columns::Number as i32);
//     //     treeview.append_column(&column);
//     // }
//
//     // Column for severities
//     // {
//     //     let renderer = gtk::CellRendererText::new();
//     //     let column = gtk::TreeViewColumn::new();
//     //     column.pack_start(&renderer, true);
//     //     column.set_title("Severity");
//     //     column.add_attribute(&renderer, "text", Columns::Severity as i32);
//     //     column.set_sort_column_id(Columns::Severity as i32);
//     //     treeview.append_column(&column);
//     // }
//
//     // let cell_renderer = gtk::CellRendererText::builder()
//
//     // Column for description
//     {
//         let renderer = gtk::CellRendererText::new();
//         let column = gtk::TreeViewColumn::new();
//         column.pack_start(&renderer, true);
//         column.set_title("Website");
//         column.set_clickable(true);
//         column.add_attribute(&renderer, "text", Columns::Website as i32);
//         column.set_sort_column_id(Columns::Website as i32);
//         treeview.append_column(&column);
//     }
//
//     {
//         let renderer = gtk::CellRendererText::new();
//         let column = gtk::TreeViewColumn::new();
//         column.pack_start(&renderer, true);
//         column.set_title("Username");
//         column.set_clickable(true);
//         column.add_attribute(&renderer, "text", Columns::Username as i32);
//         column.set_sort_column_id(Columns::Username as i32);
//         treeview.append_column(&column);
//     }
//
//     // Column for spinner
//     // {
//     //     let renderer = gtk::CellRendererSpinner::new();
//     //     let column = gtk::TreeViewColumn::new();
//     //     column.pack_start(&renderer, true);
//     //     column.set_title("Spinning");
//     //     column.add_attribute(&renderer, "pulse", Columns::Pulse as i32);
//     //     column.add_attribute(&renderer, "active", Columns::Active as i32);
//     //     treeview.append_column(&column);
//     // }
//
//     // Column for symbolic icon
//     // {
//     //     let renderer = gtk::CellRendererPixbuf::new();
//     //     let column = gtk::TreeViewColumn::new();
//     //     column.pack_start(&renderer, true);
//     //     column.set_title("Symbolic icon");
//     //     column.add_attribute(&renderer, "icon-name", Columns::Icon as i32);
//     //     column.add_attribute(&renderer, "sensitive", Columns::Sensitive as i32);
//     //     column.set_sort_column_id(Columns::Icon as i32);
//     //     treeview.append_column(&column);
//     // }
// }
//
// // fn spinner_timeout(model: &gtk::ListStore) -> Continue {
// //     let iter = model.get_iter_first().unwrap();
// //     let pulse = model
// //         .get_value(&iter, Columns::Pulse as i32)
// //         .get_some::<u32>()
// //         .unwrap_or_else(|err| {
// //             panic!(
// //                 "ListStore value for {:?} at first entry: {}",
// //                 Columns::Pulse,
// //                 err
// //             )
// //         })
// //         .wrapping_add(1);
// //
// //     model.set_value(&iter, Columns::Pulse as i32 as u32, &pulse.to_value());
// //     model.set_value(&iter, Columns::Active as i32 as u32, &true.to_value());
// //
// //     Continue(true)
// // }



//TODO Option 2

mod encryption;
mod json;
mod user_functions;
mod ui;

use encryption::encrypt_string::chacha20poly1305_encrypt_string;
use encryption::encrypt_string::chacha20poly1305_decrypt_string;
use crate::json::json_formatter::initialize_json;
use crate::user_functions::add_account::add_account;
use crate::user_functions::remove_account::remove_account;

use crate::json::json_structs::{Account, AccountList};

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, gio, glib, Grid, Label, ListBox, ScrolledWindow, TreeStore, TreeView};
use gtk::AccessibleProperty::Orientation;
use gtk::CellRendererAccelMode::Gtk;

const APP_ID: &str = "Password-Manager";

fn main() {
    // let (output, outtag) = chacha20poly1305_encrypt_string("Elegantmuffin4421121@".to_string(), "this is a message".to_string());
    //
    // chacha20poly1305_decrypt_string("Elegantmuffin4421121@".to_string(),output,outtag);
    // //initialize_json().unwrap();

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_table_ui);

    // Run the application
    app.run();
}

// fn create_table() -> ListBox {
//     let list_box = ListBox::new();
//
//     let row1 = gtk::ListBoxRow::new();
//     let hbox1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
//
//     let label1 = Label::new(Some("Website"));
//     let label2 = Label::new(Some("Username"));
//     let label3 = Label::new(Some("Password"));
//
//     hbox1.append(&label1);
//     hbox1.append(&label2);
//     hbox1.append(&label3);
//
//     row1.add(&hbox1);
//     list_box.add(&row1);
//
//     // Add additional rows with data here...
//     list_box
// }
//
// fn build_table_ui_two(app: &Application) {
//     let window = ApplicationWindow::new(app);
//
//     window.set_title(Some("Sammy's Safe"));
//     window.set_default_size(800, 800);
//
//     let grid = Grid::builder()
//         .column_spacing(5)
//         .row_spacing(10)
//         .build();
//
//     window.set_child(Some(&grid));
//
//     let list = create_table();
//
//     let sw = ScrolledWindow::builder()
//         .child(&list)
//         .vexpand(false)
//         .vexpand_set(false)
//         .max_content_height(10)
//         .build();
//
//     // column, row, width, height
//     grid.attach(&sw, 0, 0, 1, 1);
//     window.show();
// }


fn build_table_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title(Some("Sammy's Safe"));
    window.set_default_size(800, 800);

    let grid = Grid::builder()
        .column_spacing(5)
        .row_spacing(10)
        .build();

    window.set_child(Some(&grid));

    let (tree_view, _treestore) = build_tree_view();

    let sw = ScrolledWindow::builder()
        .child(&tree_view)
        .vexpand(false)
        .vexpand_set(false)
        .max_content_height(10)
        .build();

    // column, row, width, height
    grid.attach(&sw, 0, 0, 1, 1);
    window.show();
}

fn build_tree_view() -> (gtk::TreeView, gtk::TreeStore) {
    let tree_store = gtk::TreeStore::new(&[glib::Type::STRING].repeat(4));
    let tree_view = gtk::TreeView::builder()
        .hexpand(true)
        .vexpand(true)
        .enable_grid_lines(gtk::TreeViewGridLines::Both)
        .model(&tree_store)
        .build();




    for (i, title) in ["Website", "Username", "Password"]
        .iter()
        .enumerate()
    {
        let cell_renderer = gtk::CellRendererText::builder()
            .editable(true)
            .xalign(0.0)
            .single_paragraph_mode(true)
            .build();


        tree_view.insert_column_with_attributes(
            i.try_into().unwrap(),
            title,
            &cell_renderer,
            &[(&"text", i.try_into().unwrap())],
        );
        let ts = tree_store.clone();
        cell_renderer.connect_edited(move |_renderer, row, text| {
            ts.set_value(
                &ts.iter(&row).unwrap(),
                i.try_into().unwrap(),
                &text.to_value()
            );
        });
    }

    tree_store.insert_with_values(
        None,
        None,
        &[
            (0, &"one"),
            (1, &"two\ntwo\ntwo\ntwo"),
            (2, &"three"),
        ],
    );

    tree_store.insert_with_values(
      None,
        None,
        &[
            (0, &"Cum"),
            (1, &"in"),
            (2, &"my butt"),
        ]
    );

    tree_store.insert_with_values(
        None,
        None,
        &[
            (0, &""),
            (1, &""),
            (2, &""),
        ]
    );

    (tree_view, tree_store)
}

//
// //TODO Option 3
//
// //! # ListBox and ListModel Sample
// //!
// //! This sample demonstrates how to use gtk::ListBox in combination with
// //! gio::ListStore as a model with a custom row type.
// //!
// //! It sets up a gtk::ListBox containing, per row, a label, spinbutton and
// //! an edit button. The edit button allows to edit the underlying data structure
// //! and changes are taking place immediately in the listbox by making use of GObject
// //! property bindings.
// //!
// //! In addition it is possible to add new rows and delete old ones.
//
// #[macro_use]
// extern crate glib;
// extern crate gio;
// extern crate gtk;
//
// use gio::prelude::*;
// use gtk::prelude::*;
//
// use gtk::ResponseType;
//
// use std::env::args;
//
// use row_data::RowData;
//
// fn build_ui(application: &gtk::Application) {
//     let window = gtk::ApplicationWindow::new(application);
//
//     window.set_title("ListBox Model Sample");
//     window.set_border_width(10);
//     window.set_position(gtk::WindowPosition::Center);
//     window.set_default_size(320, 480);
//
//     let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
//
//     // Create our list store and specify that the type stored in the
//     // list should be the RowData GObject we define at the bottom
//     let model = gio::ListStore::new(RowData::static_type());
//
//     // And then create the UI part, the listbox and bind the list store
//     // model to it. Whenever the UI needs to show a new row, e.g. because
//     // it was notified that the model changed, it will call the callback
//     // with the corresponding item from the model and will ask for a new
//     // gtk::ListBoxRow that should be displayed.
//     //
//     // The gtk::ListBoxRow can contain any possible widgets.
//     let listbox = gtk::ListBox::new();
//     listbox.bind_model(Some(&model),
//                        clone!(@weak window => @default-panic, move |item| {
//             let box_ = gtk::ListBoxRow::new();
//             let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");
//
//             let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
//
//             // Create the label and spin button that shows the two values
//             // of the item. We bind the properties for the two values to the
//             // corresponding properties of the widgets so that they are automatically
//             // updated whenever the item is changing. By specifying SYNC_CREATE the
//             // widget will automatically get the initial value of the item set.
//             //
//             // In case of the spin button the binding is bidirectional, that is any
//             // change of value in the spin button will be automatically reflected in
//             // the item.
//             let label = gtk::Label::new(None);
//             item.bind_property("name", &label, "label")
//                 .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
//                 .build();
//             hbox.pack_start(&label, true, true, 0);
//
//             let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
//             item.bind_property("count", &spin_button, "value")
//                 .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
//                 .build();
//         hbox.pack_start(&spin_button, false, false, 0);
//
//         // When the edit button is clicked, a new modal dialog is created for editing
//         // the corresponding row
//         let edit_button = gtk::Button::with_label("Edit");
//         edit_button.connect_clicked(clone!(@weak window, @strong item => move |_| {
//             let dialog = gtk::Dialog::with_buttons(Some("Edit Item"), Some(&window), gtk::DialogFlags::MODAL,
//                 &[("Close", ResponseType::Close)]);
//             dialog.set_default_response(ResponseType::Close);
//             dialog.connect_response(|dialog, _| dialog.close());
//
//             let content_area = dialog.get_content_area();
//
//             // Similarly to the label and spin button inside the listbox, the text entry
//             // and spin button in the edit dialog are connected via property bindings to
//             // the item. Any changes will be immediately reflected inside the item and
//             // by the listbox
//             let entry = gtk::Entry::new();
//             item.bind_property("name", &entry, "text")
//                 .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
//                 .build();
//
//             // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
//             entry.connect_activate(clone!(@weak dialog => move |_| {
//                 dialog.response(ResponseType::Close);
//             }));
//             content_area.add(&entry);
//
//             let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
//             item.bind_property("count", &spin_button, "value")
//                 .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
//                 .build();
//             content_area.add(&spin_button);
//
//             dialog.show_all();
//         }));
//         hbox.pack_start(&edit_button, false, false, 0);
//
//         box_.add(&hbox);
//
//         // When a row is activated (select + enter) we simply emit the clicked
//         // signal on the corresponding edit button to open the edit dialog
//         box_.connect_activate(clone!(@weak edit_button => move |_| {
//             edit_button.emit_clicked();
//         }));
//
//         box_.show_all();
//
//         box_.upcast::<gtk::Widget>()
//     }));
//
//     let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
//     scrolled_window.add(&listbox);
//
//     let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
//
//     // The add button opens a new dialog which is basically the same as the edit
//     // dialog, except that we don't have a corresponding item yet at that point
//     // and only create it once the Ok button in the dialog is clicked, and only
//     // then add it to the model. Once added to the model, it will immediately
//     // appear in the listbox UI
//     let add_button = gtk::Button::with_label("Add");
//     add_button.connect_clicked(clone!(@weak window, @weak model => move |_| {
//             let dialog = gtk::Dialog::with_buttons(Some("Add Item"), Some(&window), gtk::DialogFlags::MODAL,
//                 &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);
//             dialog.set_default_response(ResponseType::Ok);
//
//             let content_area = dialog.get_content_area();
//
//             let entry = gtk::Entry::new();
//             entry.connect_activate(clone!(@weak dialog => move |_| {
//                 dialog.response(ResponseType::Ok);
//             }));
//             content_area.add(&entry);
//
//             let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
//             content_area.add(&spin_button);
//
//             dialog.connect_response(clone!(@weak model, @weak entry, @weak spin_button => move |dialog, resp| {
//                 let text = entry.get_text();
//                 if !text.is_empty() && resp == ResponseType::Ok {
//                     model.append(&RowData::new(&text, spin_button.get_value() as u32));
//                 }
//                 dialog.close();
//             }));
//
//             dialog.show_all();
//     }));
//
//     hbox.add(&add_button);
//
//     // Via the delete button we delete the item from the model that
//     // is at the index of the selected row. Also deleting from the
//     // model is immediately reflected in the listbox.
//     let delete_button = gtk::Button::with_label("Delete");
//     delete_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
//         let selected = listbox.get_selected_row();
//
//         if let Some(selected) = selected {
//             let idx = selected.get_index();
//             model.remove(idx as u32);
//         }
//     }));
//     hbox.add(&delete_button);
//
//     vbox.pack_start(&hbox, false, false, 0);
//     vbox.pack_start(&scrolled_window, true, true, 0);
//
//     window.add(&vbox);
//
//     for i in 0..10 {
//         model.append(&RowData::new(&format!("Name {}", i), i * 10));
//     }
//
//     window.show_all();
// }
//
// fn main() {
//     let application = gtk::Application::new(
//         Some("com.github.gtk-rs.examples.listbox-model"),
//         Default::default(),
//     )
//         .expect("Initialization failed...");
//
//     application.connect_activate(|app| {
//         build_ui(app);
//     });
//
//     application.run(&args().collect::<Vec<_>>());
// }
//
// // Our GObject subclass for carrying a name and count for the ListBox model
// //
// // Both name and count are stored in a RefCell to allow for interior mutability
// // and are exposed via normal GObject properties. This allows us to use property
// // bindings below to bind the values with what widgets display in the UI
// mod row_data {
//     use super::*;
//
//     use glib::subclass;
//     use glib::subclass::prelude::*;
//     use glib::translate::*;
//
//     // Implementation sub-module of the GObject
//     mod imp {
//         use super::*;
//         use std::cell::RefCell;
//
//         // The actual data structure that stores our values. This is not accessible
//         // directly from the outside.
//         pub struct RowData {
//             name: RefCell<Option<String>>,
//             count: RefCell<u32>,
//         }
//
//         // GObject property definitions for our two values
//         static PROPERTIES: [subclass::Property; 2] = [
//             subclass::Property("name", |name| {
//                 glib::ParamSpec::string(
//                     name,
//                     "Name",
//                     "Name",
//                     None, // Default value
//                     glib::ParamFlags::READWRITE,
//                 )
//             }),
//             subclass::Property("count", |name| {
//                 glib::ParamSpec::uint(
//                     name,
//                     "Count",
//                     "Count",
//                     0,
//                     100,
//                     0, // Allowed range and default value
//                     glib::ParamFlags::READWRITE,
//                 )
//             }),
//         ];
//
//         // Basic declaration of our type for the GObject type system
//         impl ObjectSubclass for RowData {
//             const NAME: &'static str = "RowData";
//             type ParentType = glib::Object;
//             type Instance = subclass::simple::InstanceStruct<Self>;
//             type Class = subclass::simple::ClassStruct<Self>;
//
//             glib_object_subclass!();
//
//             // Called exactly once before the first instantiation of an instance. This
//             // sets up any type-specific things, in this specific case it installs the
//             // properties so that GObject knows about their existence and they can be
//             // used on instances of our type
//             fn class_init(klass: &mut Self::Class) {
//                 klass.install_properties(&PROPERTIES);
//             }
//
//             // Called once at the very beginning of instantiation of each instance and
//             // creates the data structure that contains all our state
//             fn new() -> Self {
//                 Self {
//                     name: RefCell::new(None),
//                     count: RefCell::new(0),
//                 }
//             }
//         }
//
//         // The ObjectImpl trait provides the setters/getters for GObject properties.
//         // Here we need to provide the values that are internally stored back to the
//         // caller, or store whatever new value the caller is providing.
//         //
//         // This maps between the GObject properties and our internal storage of the
//         // corresponding values of the properties.
//         impl ObjectImpl for RowData {
//             glib_object_impl!();
//
//             fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
//                 let prop = &PROPERTIES[id];
//
//                 match *prop {
//                     subclass::Property("name", ..) => {
//                         let name = value
//                             .get()
//                             .expect("type conformity checked by `Object::set_property`");
//                         self.name.replace(name);
//                     }
//                     subclass::Property("count", ..) => {
//                         let count = value
//                             .get_some()
//                             .expect("type conformity checked by `Object::set_property`");
//                         self.count.replace(count);
//                     }
//                     _ => unimplemented!(),
//                 }
//             }
//
//             fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
//                 let prop = &PROPERTIES[id];
//
//                 match *prop {
//                     subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
//                     subclass::Property("count", ..) => Ok(self.count.borrow().to_value()),
//                     _ => unimplemented!(),
//                 }
//             }
//         }
//     }
//
//     // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
//     // binding
//     // glib_wrapper! {
//     //     pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);
//     //
//     //     match fn {
//     //         get_type => || imp::RowData::get_type().to_glib(),
//     //     }
//     // }
//     //
//     // // Constructor for new instances. This simply calls glib::Object::new() with
//     // // initial values for our two properties and then returns the new instance
//     // impl RowData {
//     //     pub fn new(name: &str, count: u32) -> RowData {
//     //         glib::Object::new(Self::static_type(), &[("name", &name), ("count", &count)])
//     //             .expect("Failed to create row data")
//     //             .downcast()
//     //             .expect("Created row data is of wrong type")
//     //     }
//     // }
// }

