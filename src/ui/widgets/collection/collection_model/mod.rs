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

use std::cmp::Ordering;

use adw::prelude::*;
use adw::subclass::prelude::*;
use chrono::prelude::*;
use gtk::{gio, glib};

use crate::ui::widgets::collection::CollectionData;

glib::wrapper! {
    pub struct CollectionModel(ObjectSubclass<imp::CollectionModel>)
        @implements gio::ListModel;
}

impl CollectionModel {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn append(&self, obj: &CollectionData) {
        let index = {
            let mut data = self.imp().0.borrow_mut();
            data.push(obj.clone());
            data.len() - 1
        };

        // (changed_index, removed_items, added_items)
        self.items_changed(index as u32, 0, 1);
    }

    pub fn remove(&self, index: u32) {
        self.imp().0.borrow_mut().remove(index as usize);
        // (changed_index, removed_items, added_items)
        self.items_changed(index, 1, 0);
    }

    pub fn sort(&self) {
        let mut data = self.imp().0.borrow_mut();
        data.sort_unstable_by(move |a, b| {
            let d1 = Utc
                .datetime_from_str(&a.property::<String>("date"), "%Y-%m-%d %H:%M:%S")
                .unwrap();
            let d2 = Utc
                .datetime_from_str(&b.property::<String>("date"), "%Y-%m-%d %H:%M:%S")
                .unwrap();

            if (a.property::<bool>("star") && !b.property::<bool>("star")) || (d1 > d2) {
                return Ordering::Greater;
            } else if d1 != d2 {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        });

        let len = data.len() as u32;
        self.items_changed(0, len, len);
    }
}

impl Default for CollectionModel {
    fn default() -> Self {
        Self::new()
    }
}
