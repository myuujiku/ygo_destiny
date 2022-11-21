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
    pub struct DraftContainer(ObjectSubclass<imp::DraftContainer>)
        @implements gtk::Widget, gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl DraftContainer {
    pub fn new(number_of_boxes: usize, max_selected: usize) -> Self {
        let o: Self = glib::Object::builder().build();

        o.imp().number_of_boxes.set(number_of_boxes).unwrap();
        o.imp().max_selected.set(max_selected).unwrap();

        return o;
    }

    pub fn populate_boxes(&self, card_ids: &Vec<Vec<u32>>) {
        self.imp().populate_boxes(card_ids);
    }
}
