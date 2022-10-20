mod ui;

use adw::prelude::*;
use gtk::gio;

const APP_ID: &str = "com.myujiku.ygod";

fn main() {
    // Resources
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    // Application init
    let app = adw::Application::new(Some(APP_ID), Default::default());

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let _main_menu = ui::main_menu::MainMenu::new();
    let window = ui::window::Window::new(app);

    window.present();
}
