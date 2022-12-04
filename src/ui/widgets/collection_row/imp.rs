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

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/collection_row.ui")]
pub struct CollectionRow {
    #[template_child]
    pub star_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionRow {
    const NAME: &'static str = "YGOCollectionRow";
    type Type = super::CollectionRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CollectionRow {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {
        // Unparent all direct children
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CollectionRow {}
impl ActionRowImpl for CollectionRow {}
impl ListBoxRowImpl for CollectionRow {}
impl PreferencesRowImpl for CollectionRow {}
