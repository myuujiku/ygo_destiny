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
use chrono::prelude::*;
use gtk::{Align, Orientation};
use relm4::prelude::*;
use rust_i18n::t;
use ygod_core::user_data::{Collection, LAST_CHANGED_FORMAT};

use crate::components::{ViewControllerInput, ViewControllerPage};

#[derive(Debug)]
pub enum CreateCollectionPageInput {
    Save,
}

pub struct CreateCollectionPage;

#[relm4::component(pub)]
impl Component for CreateCollectionPage {
    type Init = ();
    type Input = CreateCollectionPageInput;
    type Output = ViewControllerInput;
    type CommandOutput = ();
    type Widgets = CreateCollectionPageWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: Orientation::Vertical,

            adw::HeaderBar {
                set_title_widget: Some(&adw::WindowTitle::new(
                    &t!("pages.create_collection.title"),
                    "",
                )),
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",
                    connect_clicked[sender] => move |_| {
                        sender.output(ViewControllerInput::ClosePage).unwrap();
                    },
                },
                pack_end = &gtk::Button {
                    set_label: &t!("pages.create_collection.create_button.label"),
                    add_css_class: "suggested-action",
                    connect_clicked => CreateCollectionPageInput::Save,
                },
            },
            adw::Clamp {
                set_orientation: Orientation::Horizontal,
                set_maximum_size: 800,

                gtk::Box::new(Orientation::Vertical, 6) {
                    set_hexpand: true,
                    set_vexpand: true,
                    set_valign: Align::Center,
                    set_margin_all: 6,

                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: &t!("pages.create_collection.draft_settings.label"),
                            set_activatable_widget: Some(&gtk::Label::new(None)),

                            add_suffix: &gtk::Image::builder().icon_name("go-next-symbolic").build(),
                        },
                    },
                    adw::PreferencesGroup {
                        #[name = "name_entry"]
                        adw::EntryRow {
                            set_title: &t!("pages.create_collection.name_entry.title"),
                        },
                        #[name = "desc_entry"]
                        adw::EntryRow {
                            set_title: &t!("pages.create_collection.desc_entry.title"),
                        },
                    },
                    adw::PreferencesGroup {
                        adw::ActionRow {
                            set_title: &t!("pages.create_collection.starred_switch.title"),
                            set_subtitle: &t!("pages.create_collection.starred_switch.description"),

                            add_suffix = starred_switch = &gtk::Switch {
                                set_valign: Align::Center,
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut CreateCollectionPageWidgets,
        input: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match input {
            CreateCollectionPageInput::Save => {
                let mut collection = Collection::default();

                // Set collection name
                collection.meta_data.name = widgets.name_entry.text().to_string();

                // Set collection description
                collection.meta_data.description = widgets.desc_entry.text().to_string();

                // Set collection pinned
                collection.meta_data.pinned = widgets.starred_switch.state();

                // Set date
                let date = Utc::now().format(LAST_CHANGED_FORMAT).to_string();
                collection.meta_data.last_changed = date.clone();

                collection.save(&date);

                sender
                    .output(ViewControllerInput::ReplacePage(
                        ViewControllerPage::Collection(date),
                    ))
                    .unwrap();
            }
        }
    }
}
