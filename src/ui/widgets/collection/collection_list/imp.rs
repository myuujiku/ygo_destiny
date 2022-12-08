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

use crate::ui::widgets::collection::{CollectionData, CollectionModel, CollectionRow};

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

        let collection_model = CollectionModel::new();

        collection_model.append(&CollectionData::new(
            "",
            "Collection 1",
            "desc 1",
            "2022-12-07 14:01:09",
            false,
        ));
        collection_model.append(&CollectionData::new(
            "",
            "Collection 2",
            "desc 2",
            "2022-12-07 14:02:32",
            false,
        ));
        collection_model.append(&CollectionData::new(
            "",
            "Collection 3",
            "desc 3",
            "2022-12-07 14:07:40",
            true,
        ));
        collection_model.append(&CollectionData::new(
            "",
            "Collection 4",
            "desc 4",
            "2022-12-08 00:51:11",
            false,
        ));

        collection_model.sort();

        self.list_box
            .bind_model(Some(&collection_model), move |item| {
                CollectionRow::new(
                    item.downcast_ref::<CollectionData>()
                        .expect("CollectionData is of wrong type."),
                )
                .upcast::<gtk::Widget>()
            });
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

// TODO: Implement meta data
// see std::path::Path::{read_dir, metadata} and std::fs::Metadata::modified
