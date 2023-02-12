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

use gtk::{gio, prelude::*};
use relm4::prelude::*;
use ygod_core::APP_ID;

use ygo_destiny::App;

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resources.");

    let main_app = relm4::main_application();
    main_app.set_application_id(Some(APP_ID));
    main_app.set_resource_base_path(Some("/com/myujiku/ygo_destiny/"));

    let app = RelmApp::with_app(main_app);
    app.run::<App>(());
}
