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
use gtk::glib;

glib::wrapper! {
    pub struct RowSplitBox(ObjectSubclass<imp::RowSplitBox>)
        @implements gtk::Widget, gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RowSplitBox {
    pub fn new(cell_width: i32, cell_height: i32, h_spacing: i32, v_spacing: i32) -> Self {
        let o: Self = glib::Object::builder().build();

        o.imp().cell_width.set(cell_width).unwrap();
        o.imp().cell_height.set(cell_height).unwrap();
        o.imp().h_spacing.set(h_spacing).unwrap();
        o.imp().v_spacing.set(v_spacing).unwrap();

        return o;
    }

    pub fn insert(&self, widget: gtk::Image) {
        widget.set_parent(self);
        self.imp().children.borrow_mut().push(widget);
    }
}
