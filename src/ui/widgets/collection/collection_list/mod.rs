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

mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use chrono::prelude::*;
use gtk::glib;

use crate::logic::user_data::{MetaData, LAST_CHANGED_FORMAT};

glib::wrapper! {
    pub struct CollectionList(ObjectSubclass<imp::CollectionList>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl CollectionList {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn new_collection_meta_data(&self) -> MetaData {
        let imp = self.imp();
        MetaData {
            name: imp.name_entry.text().to_string(),
            description: imp.desc_entry.text().to_string(),
            pinned: imp.starred_switch.is_active(),
            last_changed: format!("{}", Utc::now().format(LAST_CHANGED_FORMAT)),
        }
    }
}
