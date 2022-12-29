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
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

use crate::logic::user_data::ProgressiveCollection;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/edit_collection_page.ui")]
pub struct EditCollectionPage {
    pub name: RefCell<String>,
    pub collection: RefCell<ProgressiveCollection>,
    #[template_child]
    pub window_title: TemplateChild<adw::WindowTitle>,
    #[template_child]
    pub back_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for EditCollectionPage {
    const NAME: &'static str = "YGOEditCollectionPage";
    type Type = super::EditCollectionPage;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EditCollectionPage {}
impl WidgetImpl for EditCollectionPage {}
impl BoxImpl for EditCollectionPage {}
