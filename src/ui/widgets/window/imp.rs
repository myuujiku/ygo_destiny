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

use std::sync::Mutex;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/YGODestiny/templates/window.ui")]
pub struct Window {
    #[template_child]
    pub leaflet: TemplateChild<adw::Leaflet>,
    #[template_child]
    pub toast_overlay: TemplateChild<adw::ToastOverlay>,
    pub update_version: Mutex<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "YGOWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn dispose(&self) {
        self.toast_overlay.unparent();
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl AdwApplicationWindowImpl for Window {}
