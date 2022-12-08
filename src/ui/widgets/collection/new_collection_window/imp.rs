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
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/new_collection_window.ui")]
pub struct NewCollectionWindow {}

#[glib::object_subclass]
impl ObjectSubclass for NewCollectionWindow {
    const NAME: &'static str = "YGONewCollectionWindow";
    type Type = super::NewCollectionWindow;
    type ParentType = adw::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj:  &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for NewCollectionWindow {}
impl WidgetImpl for NewCollectionWindow {}
impl WindowImpl for NewCollectionWindow {}
impl AdwWindowImpl for NewCollectionWindow {}

#[gtk::template_callbacks]
impl NewCollectionWindow {}

