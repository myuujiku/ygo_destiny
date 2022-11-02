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
use gtk::glib;

glib::wrapper! {
    pub struct UpdatePage(ObjectSubclass<imp::UpdatePage>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl UpdatePage {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn get_progress_bar(&self) -> &gtk::TemplateChild<gtk::ProgressBar> {
        return &self.imp().progress_bar;
    }
}
