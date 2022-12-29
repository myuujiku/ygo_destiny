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

use adw::subclass::prelude::*;
use gtk::{glib, TemplateChild};

use crate::logic::user_data::ProgressiveCollection;

glib::wrapper! {
    pub struct EditCollectionPage(ObjectSubclass<imp::EditCollectionPage>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl EditCollectionPage {
    pub fn new(name: &String) -> Self {
        let obj: Self = glib::Object::new(&[]);
        let imp = obj.imp();
        *imp.name.borrow_mut() = name.to_string();
        *imp.collection.borrow_mut() = ProgressiveCollection::from_name(name.to_string());

        imp.window_title
            .set_title(&format!("{}", imp.collection.borrow().meta_data.name));

        obj
    }
}
