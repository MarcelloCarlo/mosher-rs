use gtk::gio::ffi::GAsyncReadyCallback;
use gtk::gio::{Cancellable, ListStore};
use gtk::{Application, ApplicationWindow, FileDialog, glib};
use gtk::{Box, Button, FileFilter, prelude::*};

const APP_ID: &str = "com.marcellocarlo.mosher-rs";
const APP_NAME: &str = "Mosher-RS";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create Input File Fields

    let box_main = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let btn_mosh_it = Button::builder()
        .label("Mosh It!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let btn_select_image = Button::builder()
        .label("Select Image")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let chk_linear_mosh = gtk::CheckButton::builder()
        .halign(gtk::Align::Start)
        .label("Linear Mosh")
        .build();

    let chk_random_mosh = gtk::CheckButton::builder()
        .halign(gtk::Align::End)
        .label("Random Mosh")
        .build();

    box_main.append(&btn_select_image);
    box_main.append(&chk_linear_mosh);
    box_main.append(&chk_random_mosh);
    box_main.append(&btn_mosh_it);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title(APP_NAME)
        .child(&box_main)
        .default_width(400)
        .default_height(300)
        .build();

    // Present window
    window.present();

    btn_mosh_it.connect_clicked(|btn_mosh_it| {
        btn_mosh_it.set_label("Clicked Mosh It!");
    });

    btn_select_image.connect_clicked(move |_| {
        select_mosh_file(&window);
    });
}

fn select_mosh_file(window: &ApplicationWindow) {
    let filters = ListStore::new::<FileFilter>();
    let file_filter = FileFilter::new();
    file_filter.set_name(Some("Image Files"));
    file_filter.add_mime_type("image/png");
    file_filter.add_mime_type("image/jpeg");
    filters.append(&file_filter);

    let file_chooser = FileDialog::builder()
        .title("Open File")
        .accept_label("Open")
        .modal(true)
        .filters(&filters)
        .build();

    file_chooser.open(Some(window), Cancellable::NONE, move |result| {
        result
            .map(|file| {
                // Here you can handle the selected file
                println!("Selected file: {}", file.parse_name());
            })
            .unwrap_or_else(|e| {
                eprintln!("Error selecting file: {}", e);
            });
    });
}
