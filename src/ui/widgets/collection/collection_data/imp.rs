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

use std::cell::{Cell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
use gtk::glib;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct CollectionData {
    file: RefCell<String>,
    name: RefCell<String>,
    desc: RefCell<String>,
    date: RefCell<String>,
    filter: RefCell<String>,
    star: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionData {
    const NAME: &'static str = "YGOCollectionData";
    type Type = super::CollectionData;
}

impl ObjectImpl for CollectionData {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("file").build(),
                ParamSpecString::builder("name").build(),
                ParamSpecString::builder("desc").build(),
                ParamSpecString::builder("date").build(),
                ParamSpecString::builder("filter").build(),
                ParamSpecBoolean::builder("star").build(),
            ]
        });
        return PROPERTIES.as_ref();
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "file" => {
                let val = value.get().expect("Expected `String`.");
                self.file.replace(val);
            }
            "name" => {
                let val = value.get().expect("Expected `String`.");
                self.name.replace(val);
            }
            "desc" => {
                let val = value.get().expect("Expected `String`.");
                self.desc.replace(val);
            }
            "date" => {
                let val = value.get().expect("Expected `String`.");
                self.date.replace(val);
            }
            "filter" => {
                let val = value.get().expect("Expected `String`.");
                self.filter.replace(val);
            }
            "star" => {
                let val = value.get().expect("Expected `Boolean`.");
                self.star.replace(val);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "file" => self.name.borrow().to_value(),
            "name" => self.name.borrow().to_value(),
            "desc" => self.desc.borrow().to_value(),
            "date" => self.date.borrow().to_value(),
            "filter" => self.filter.borrow().to_value(),
            "star" => self.star.get().to_value(),
            _ => unimplemented!(),
        }
    }
}
