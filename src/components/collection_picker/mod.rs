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

mod collection_entry;

use std::cmp::Ordering;
use std::convert::identity;

use adw::prelude::*;
use chrono::prelude::*;
use gtk::{Align, Orientation};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use ygod_core::user_data::{collection::LAST_CHANGED_FORMAT, Collection};

use crate::components::ViewControllerInput;
use collection_entry::{CollectionData, CollectionEntry};

pub struct CollectionPicker {
    collection_entries: FactoryVecDeque<CollectionEntry>,
}

#[relm4::component(pub)]
impl SimpleComponent for CollectionPicker {
    type Init = ();
    type Input = ();
    type Output = ViewControllerInput;
    type Widgets = CollectionPickerWidgets;

    view! {
        #[root]
        gtk::ScrolledWindow {
            set_min_content_height: 200,
            set_hscrollbar_policy: gtk::PolicyType::Never,

            adw::Clamp {
                set_orientation: Orientation::Horizontal,
                set_maximum_size: 800,

                gtk::Box::new(Orientation::Vertical, 6) {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_valign: Align::Center,
                    set_margin_all: 6,

                    gtk::Label::new(Some("Collection")) {
                        add_css_class: "heading",
                        set_halign: Align::Start,
                    },
                    gtk::Box::new(Orientation::Horizontal, 6) {
                        gtk::SearchEntry {
                            set_hexpand: true,
                        },
                        gtk::Button {
                            set_icon_name: "list-add",
                            add_css_class: "circular",
                        }
                    },
                    #[local_ref]
                    collection_entry_box -> gtk::ListBox {
                        add_css_class: "boxed-list",
                    }
                }
            }
        }
    }

    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let collection_names = Collection::get_names();
        let mut collection_entries_components = Vec::new();
        for collection_name in collection_names {
            let meta_data = Collection::get_metadata_from(&collection_name);
            collection_entries_components.push(CollectionData::new(collection_name, meta_data));
        }
        collection_entries_components
            .sort_unstable_by(|first, second| {
                let first_date = Utc
                    .datetime_from_str(&first.meta_data.last_changed, LAST_CHANGED_FORMAT)
                    .unwrap();
                let second_date = Utc
                    .datetime_from_str(&second.meta_data.last_changed, LAST_CHANGED_FORMAT)
                    .unwrap();

                if first.meta_data.pinned == second.meta_data.pinned {
                    if first_date < second_date {
                        Ordering::Greater
                    } else if first_date != second_date {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                } else {
                    match first.meta_data.pinned {
                        true => Ordering::Less,
                        false => Ordering::Greater,
                    }
                }
            });

        let mut collection_entries =
            FactoryVecDeque::from_vec(collection_entries_components, gtk::ListBox::default(), sender.input_sender());

        let model = Self { collection_entries };
        let collection_entry_box = model.collection_entries.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
