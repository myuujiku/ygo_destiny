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

use adw::prelude::*;
use chrono::prelude::*;
use gtk::{Align, Orientation};
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;
use rust_i18n::t;
use ygod_core::user_data::{collection::LAST_CHANGED_FORMAT, Collection};

use crate::components::ViewControllerInput;
use collection_entry::{
    CollectionData, CollectionEntry, CollectionEntryInput, CollectionEntryOutput,
};

pub struct CollectionPicker {
    collection_entries: FactoryVecDeque<CollectionEntry>,
}

#[relm4::component(pub)]
impl SimpleComponent for CollectionPicker {
    type Init = ();
    type Input = CollectionEntryOutput;
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

                    gtk::Label::new(Some(&t!("collection_picker_heading"))) {
                        add_css_class: "heading",
                        set_halign: Align::Start,
                    },
                    gtk::Box::new(Orientation::Horizontal, 6) {
                        gtk::SearchEntry {
                            set_hexpand: true,
                            connect_search_changed[sender] => move |search_entry| {
                                sender.input(CollectionEntryOutput::FilterBy(search_entry.text().to_string()));
                            },
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
        collection_entries_components.sort_unstable_by(|first, second| {
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

        let collection_entries = FactoryVecDeque::from_vec(
            collection_entries_components,
            gtk::ListBox::default(),
            sender.input_sender(),
        );

        let model = Self { collection_entries };
        let collection_entry_box = model.collection_entries.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {
            CollectionEntryOutput::SortUp(dynamic_index) => {
                let mut index = dynamic_index.current_index();
                let entry = self.collection_entries.get(index).unwrap();
                let entry_date = Utc
                    .datetime_from_str(&entry.last_modified, LAST_CHANGED_FORMAT)
                    .unwrap();
                loop {
                    if index == 0 {
                        break;
                    }

                    if let Some(other) = self.collection_entries.get(index - 1) {
                        let other_date = Utc
                            .datetime_from_str(&other.last_modified, LAST_CHANGED_FORMAT)
                            .unwrap();
                        if other.pinned.get() && entry_date < other_date {
                            break;
                        }

                        self.collection_entries.guard().move_to(index, index - 1);
                        index -= 1;
                    } else {
                        break;
                    }
                }
            }
            CollectionEntryOutput::SortDown(dynamic_index) => {
                let mut index = dynamic_index.current_index();
                let entry = self.collection_entries.get(index).unwrap();
                let entry_date = Utc
                    .datetime_from_str(&entry.last_modified, LAST_CHANGED_FORMAT)
                    .unwrap();
                loop {
                    if index == usize::MAX {
                        break;
                    }

                    if let Some(other) = self.collection_entries.get(index + 1) {
                        let other_date = Utc
                            .datetime_from_str(&other.last_modified, LAST_CHANGED_FORMAT)
                            .unwrap();
                        if !other.pinned.get() && entry_date > other_date {
                            break;
                        }

                        self.collection_entries.guard().move_to(index, index + 1);
                        index += 1;
                    } else {
                        break;
                    }
                }
            }
            CollectionEntryOutput::FilterBy(text) => {
                let text_lower = text.to_lowercase();
                let sort_case_sensitive: bool = text != text_lower;

                let mut i = 0;
                loop {
                    if let Some(entry) = self.collection_entries.get(i) {
                        let matches: bool = if sort_case_sensitive {
                            entry.name.contains(&text) || entry.description.contains(&text)
                        } else {
                            entry.name.to_lowercase().contains(&text_lower)
                                || entry.description.to_lowercase().contains(&text_lower)
                        };

                        self.collection_entries
                            .send(i, CollectionEntryInput::SetVisible(matches));

                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            CollectionEntryOutput::OpenCollection(file_name) => {
                sender.output(ViewControllerInput::AddPage(
                    gtk::Label::new(Some(&file_name)).upcast::<gtk::Widget>(),
                )).unwrap();
            }
        }
    }
}
