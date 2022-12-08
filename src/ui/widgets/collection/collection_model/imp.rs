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

use std::cell::RefCell;

use adw::prelude::*;
use gio::subclass::prelude::*;
use gtk::{gio, glib};

use crate::ui::widgets::collection::CollectionData;

#[derive(Debug, Default)]
pub struct CollectionModel(pub(super) RefCell<Vec<CollectionData>>);

#[glib::object_subclass]
impl ObjectSubclass for CollectionModel {
    const NAME: &'static str = "YGOCollectionModel";
    type Type = super::CollectionModel;
    type Interfaces = (gio::ListModel,);
}

impl ObjectImpl for CollectionModel {}

impl ListModelImpl for CollectionModel {
    fn item_type(&self) -> glib::Type {
        CollectionData::static_type()
    }

    fn n_items(&self) -> u32 {
        self.0.borrow().len() as u32
    }

    fn item(&self, position: u32) -> Option<glib::Object> {
        self.0
            .borrow()
            .get(position as usize)
            .map(|o| o.clone().upcast::<glib::Object>())
    }
}
