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
use gtk::{Align, Orientation};
use relm4::prelude::*;
use rust_i18n::t;
use ygod_core::user_data::Collection;

use crate::components::ViewControllerInput;

#[derive(Debug)]
pub enum CreateCollectionPageInput {
    Save,
}

pub struct CreateCollectionPage {
    pub collection: Collection,
}

#[relm4::component(pub)]
impl SimpleComponent for CreateCollectionPage {
    type Init = ();
    type Input = CreateCollectionPageInput;
    type Output = ViewControllerInput;
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
                        adw::EntryRow {
                            set_title: &t!("pages.create_collection.name_entry.title"),
                        },
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
        let collection = Collection::default();

        let model = Self { collection };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>) {
        match input {
            CreateCollectionPageInput::Save => {}
        }
    }
}
