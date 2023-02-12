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
use gtk::Orientation;
use relm4::prelude::*;
use rust_i18n::t;
use ygod_core::user_data::Collection;

use crate::components::ViewControllerInput;
use crate::templates::BackButton;

pub struct CollectionPage {
    pub file_name: String,
    pub collection: Collection,
}

#[relm4::component(pub)]
impl SimpleComponent for CollectionPage {
    type Init = String;
    type Input = ();
    type Output = ViewControllerInput;
    type Widgets = CollectionPageWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: Orientation::Vertical,

            adw::HeaderBar {
                set_title_widget: Some(&adw::WindowTitle::new(
                    &t!("pages.collection.title", name = &model.collection.meta_data.name),
                    "",
                )),
                #[template]
                pack_start = &BackButton {
                    set_exit_message: (sender.output_sender(), ViewControllerInput::ClosePage),
                },
            },
            #[name = "stack"]
            adw::ViewStack {
                add = &gtk::Label {
                    set_vexpand: true,
                    set_label: "Draft",
                } -> {
                    set_title: Some("Draft"),
                    set_icon_name: Some("draft-symbolic"),
                },
                add = &gtk::Label {
                    set_label: "Decks",
                } -> {
                    set_title: Some("Decks"),
                    set_icon_name: Some("deck-edit-symbolic"),
                },
                add = &gtk::Label {
                    set_label: "Banlist",
                } -> {
                    set_title: Some("Banlist"),
                    set_icon_name: Some("banlist-symbolic"),
                },
                add = &gtk::Label {
                    set_label: "Settings",
                } -> {
                    set_title: Some("Settings"),
                    set_icon_name: Some("settings-symbolic"),
                },
            },
            adw::ViewSwitcherBar {
                set_stack: Some(&stack),
                set_reveal: true,
            },
        }
    }

    fn init(
        file_name: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let collection = Collection::from_name(&file_name);

        let model = Self {
            file_name,
            collection,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
