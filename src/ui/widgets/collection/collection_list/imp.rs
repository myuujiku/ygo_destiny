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
#[template(resource = "/com/myujiku/ygo_destiny/templates/collection_list.ui")]
pub struct CollectionList {
    #[template_child]
    pub list_box: TemplateChild<gtk::ListBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionList {
    const NAME: &'static str = "YGOCollectionList";
    type Type = super::CollectionList;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CollectionList {
    fn constructed(&self) {
        self.parent_constructed();

        self.list_box.add_css_class("boxed-list");

        self.list_box
            .append(&crate::ui::widgets::collection::CollectionRow::new());
        self.list_box
            .append(&crate::ui::widgets::collection::CollectionRow::new());
        self.list_box
            .append(&crate::ui::widgets::collection::CollectionRow::new());
    }

    fn dispose(&self) {
        // Unparent all direct children
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CollectionList {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
    }
}

impl BoxImpl for CollectionList {}

// TODO: Implement a sort_func for ListBox
//
// (a, b) -> gtk::Ordering
//
// if (pinned(a) && !pinned(b)) || date(a) > date(b) {
//     return Ordering::Larger;
// } else if date(a) != date(b) {
//     return Ordering::Smaller;
// } else {
//     return Ordering::Equal;
// }
//
// pinned is actually a Vec::contains call.
// date should give a value that can be compared and is the last modified date of the file
// containing the collection.
// see std::path::Path::{read_dir, metadata} and std::fs::Metadata::modified
