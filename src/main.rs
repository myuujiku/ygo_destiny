mod ui;

use adw::prelude::*;
use gtk::gio;
use ui::window::Window;

const APP_ID: &str = "com.myujiku.ygod";

fn main() {
    // Resources
    gio::resources_register_include!("ygod.gresource")
        .expect("Failed to register resources.");

    // Application init
    let app = adw::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);

    window.present();
}
