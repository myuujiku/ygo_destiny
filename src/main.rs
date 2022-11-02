/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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

    window.present();
}
