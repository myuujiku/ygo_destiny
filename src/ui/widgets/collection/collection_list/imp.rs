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
use chrono::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate, Ordering};

use crate::logic::user_data::{ProgressiveCollection, LAST_CHANGED_FORMAT};
use crate::ui::widgets::collection::{CollectionData, CollectionModel, CollectionRow};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/collection_list.ui")]
pub struct CollectionList {
    #[template_child]
    pub list_box: TemplateChild<gtk::ListBox>,
    #[template_child]
    pub search_bar: TemplateChild<gtk::SearchEntry>,
    #[template_child]
    pub add_collection_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub options_button: TemplateChild<adw::ActionRow>,
    #[template_child]
    pub create_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub popover: TemplateChild<gtk::Popover>,
    #[template_child]
    pub name_entry: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub desc_entry: TemplateChild<adw::EntryRow>,
    #[template_child]
    pub starred_switch: TemplateChild<gtk::Switch>,
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

        // Add the libadwaita `boxed-list` style
        self.list_box.add_css_class("boxed-list");

        let collection_model = CollectionModel::new();

        for collection_name in ProgressiveCollection::get_names() {
            let meta_data = ProgressiveCollection::get_metadata_from(&collection_name);
            collection_model.append(&CollectionData::new(
                &collection_name,
                &meta_data.name,
                &meta_data.description,
                &meta_data.last_changed,
                meta_data.pinned,
            ));
        }

        let filter = gtk::CustomFilter::new(move |obj| {
            let data = obj
                .downcast_ref::<CollectionData>()
                .expect("Object not of type `CollectionData`.");

            let filter = data.property::<String>("filter");

            // TODO: Consider making description search optional
            return data
                .property::<String>("name")
                .to_lowercase()
                .contains(filter.as_str())
                || data
                    .property::<String>("desc")
                    .to_lowercase()
                    .contains(filter.as_str());
        });

        let filter_model = gtk::FilterListModel::new(Some(&collection_model), Some(&filter));

        let sorter = gtk::CustomSorter::new(move |obj1, obj2| {
            let a = obj1
                .downcast_ref::<CollectionData>()
                .expect("Object not of type `CollectionData`.");
            let b = obj2
                .downcast_ref::<CollectionData>()
                .expect("Object not of type `CollectionData`.");

            let d1 = Utc
                .datetime_from_str(&a.property::<String>("date"), LAST_CHANGED_FORMAT)
                .unwrap();
            let d2 = Utc
                .datetime_from_str(&b.property::<String>("date"), LAST_CHANGED_FORMAT)
                .unwrap();

            let a_starred = a.property::<bool>("star");
            let b_starred = b.property::<bool>("star");

            if a_starred == b_starred {
                if d1 < d2 {
                    return Ordering::Larger;
                } else if d1 != d2 {
                    return Ordering::Smaller;
                } else {
                    return Ordering::Equal;
                }
            } else {
                return match a_starred {
                    true => Ordering::Smaller,
                    false => Ordering::Larger,
                };
            }
        });

        let sort_model = gtk::SortListModel::new(Some(&filter_model), Some(&sorter));

        self.search_bar.connect_search_changed(
            glib::clone!(@weak filter, @weak collection_model => move |search_bar| {
                let text = search_bar.text().to_lowercase();

                let mut i = 0;
                loop {
                    let item = collection_model.item(i);

                    if item.is_none() {
                        break;
                    }

                    item.unwrap().set_property("filter", text.clone());
                    i += 1;
                }

                filter.changed(gtk::FilterChange::Different);
            }),
        );

        self.list_box.bind_model(Some(&sort_model), move |item| {
            let row = CollectionRow::new(
                item.downcast_ref::<CollectionData>()
                    .expect("CollectionData is of wrong type."),
            );

            row.connect_closure(
                "pin-action",
                false,
                glib::closure_local!(@weak-allow-none sorter => move |_: CollectionRow| {
                    sorter.unwrap().changed(gtk::SorterChange::Different);
                }),
            );

            return row.upcast::<gtk::Widget>();
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
