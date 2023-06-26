use std::cmp::Ordering;

use adw::{gtk::Align, prelude::*};
use chrono::prelude::*;
use gtk::Orientation;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::ui::{
    components::{CollectionData, CollectionEntry, CollectionEntryInput, CollectionEntryOutput},
    templates,
};
use crate::user_data::collection::{Collection, LAST_CHANGED_FORMAT};

#[derive(Debug)]
pub enum AppInput {
    CollectionEvent(CollectionEntryOutput),
}

pub struct App {
    collection_entries: FactoryVecDeque<CollectionEntry>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        adw::Window {
            set_width_request: 640,
            set_height_request: 480,
            add_css_class: "devel",

            #[name = "main_leaflet"]
            adw::Leaflet {
                set_can_unfold: false,
                set_transition_type: adw::LeafletTransitionType::Slide,

                #[name = "collection_picker_leaflet"]
                adw::Leaflet {
                    set_can_unfold: false,
                    set_transition_type: adw::LeafletTransitionType::Slide,

                    #[template]
                    templates::Page {
                        #[template_child]
                        back_button {
                            set_visible: false,
                        },

                        adw::Clamp {
                            set_visible: model.collection_entries.is_empty(),
                            set_orientation: Orientation::Horizontal,
                            set_maximum_size: 800,

                            adw::StatusPage {
                                set_title: "no_collection_found",
                                set_vexpand: true,
                                #[wrap(Some)]
                                set_child = &gtk::Button {
                                    add_css_class: "suggested-action",
                                    add_css_class: "pill",
                                    set_label: "create_collection",
                                    set_halign: Align::Center,
                                    connect_clicked[sender] => move |_| {}
                                }
                            }
                        },
                        gtk::ScrolledWindow {
                            set_visible: !model.collection_entries.is_empty(),
                            set_min_content_height: 200,
                            set_hscrollbar_policy: gtk::PolicyType::Never,
                            connect_unrealize => AppInput::CollectionEvent(
                                CollectionEntryOutput::SaveChanges
                            ),

                            adw::Clamp {
                                set_orientation: Orientation::Horizontal,
                                set_maximum_size: 800,

                                gtk::Box::new(Orientation::Vertical, 6) {
                                   set_hexpand: true,
                                   set_vexpand: true,
                                   set_valign: Align::Center,
                                   set_margin_all: 6,

                                    gtk::Label::new(Some("collections")) {
                                        add_css_class: "heading",
                                        set_halign: Align::Start,
                                    },
                                    gtk::Box::new(Orientation::Horizontal, 6) {
                                        gtk::SearchEntry {
                                            set_hexpand: true,
                                            connect_search_changed[sender] => move |search_entry| {
                                                sender.input(AppInput::CollectionEvent(CollectionEntryOutput::FilterBy(search_entry.text().to_string())));
                                            },
                                        },
                                        gtk::Button {
                                            set_icon_name: "list-add",
                                            add_css_class: "circular",
                                            connect_clicked[sender] => move |_| {},
                                        },
                                    },
                                   #[local_ref]
                                   collection_entry_box -> gtk::ListBox {
                                        add_css_class: "boxed-list",
                                   }
                                }
                            }
                        }
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

        let collection_entries = FactoryVecDeque::from_iter(
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
            AppInput::CollectionEvent(input) => match input {
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
                    let mut matched: bool = false;

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

                            if matches {
                                matched = true;
                            }

                            i += 1;
                        } else {
                            break;
                        }
                    }

                    self.collection_entries.widget().set_visible(matched);
                }
                CollectionEntryOutput::OpenCollection(file_name) => {
                    todo!();
                }
                CollectionEntryOutput::SaveChanges => {
                    for entry in self.collection_entries.iter() {
                        if entry.pinned.has_changed() {
                            let mut collection = Collection::from_name(&entry.file);
                            collection.meta_data.pinned = entry.pinned.get();
                            collection.save(&entry.file);
                        }
                    }
                }
            },
        }
    }
}
