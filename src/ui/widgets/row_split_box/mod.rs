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

use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

glib::wrapper! {
    pub struct RowSplitBox(ObjectSubclass<imp::RowSplitBox>)
        @implements gtk::Widget, gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RowSplitBox {
    pub fn new(cell_width: i32, cell_height: i32, h_spacing: i32, v_spacing: i32) -> Self{
        let b: Self = glib::Object::builder().build();

        *b.imp().cell_width.lock().unwrap() = cell_width;
        *b.imp().cell_height.lock().unwrap() = cell_height;
        *b.imp().h_spacing.lock().unwrap() = h_spacing;
        *b.imp().v_spacing.lock().unwrap() = v_spacing;

        return b;
    }

    pub fn insert(&self, widget: gtk::Image) {
        // TODO: try moving this functionality into imp
        &widget.set_parent(self);
        self.imp().children.lock().unwrap().push(RefCell::new(widget));
        //self.imp().children.lock().unwrap().last().unwrap().borrow().set_parent(self);
    }
}
