use adw::prelude::*;
use gtk::{gio, glib};

use ygo_destiny::logic::utils::http;
use ygo_destiny::ui::widgets::window::Window;

const APP_ID: &str = "com.myujiku.ygo_destiny";

fn main() {
    // Preload external data
    http::load_local_data();

    // Register resources
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    // Initialise application
    let app = adw::Application::new(Some(&APP_ID), Default::default());

    // Build the application window
    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = Window::new(app);

    let update_action = gio::SimpleAction::new("update_data", None);
    update_action.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.update();
    }));
    window.add_action(&update_action);

    window.show_update_notification();

    use ygo_destiny::logic::ext_data::image_dl::cards;
    cards::download_missing_cards(cards::ImageType::Big);

    window.present();
}
