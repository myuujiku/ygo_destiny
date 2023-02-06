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
// use ygod_core::user_data::DraftSettings;

use crate::components::ViewControllerInput;

#[derive(Debug)]
pub enum DraftSettingsPageInput {}

pub struct DraftSettingsPage {
    // settings: Vec<(DraftSettings, usize)>,
}

#[relm4::component(pub)]
impl SimpleComponent for DraftSettingsPage {
    type Init = ();
    type Input = DraftSettingsPageInput;
    type Output = ViewControllerInput;
    type Widgets = DraftSettingsPageWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: Orientation::Vertical,

            adw::Leaflet {
                set_can_unfold: false,

                adw::HeaderBar {
                    set_title_widget: Some(&adw::WindowTitle::new("Draft Settings", "")),
                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous-symbolic",
                        connect_clicked[sender] => move |_| {
                            sender.output(ViewControllerInput::ClosePage).unwrap();
                        },
                    },
                }
            },
            gtk::Box {
                set_orientation: Orientation::Vertical,
                set_vexpand: true,
            },
            gtk::Box {
                set_orientation: Orientation::Horizontal,
                add_css_class: "toolbar",

                gtk::Separator {
                    add_css_class: "spacer",
                    set_hexpand: true,
                },
                gtk::Button {
                    set_icon_name: "list-add",
                },
                gtk::Separator {
                    add_css_class: "spacer",
                    set_hexpand: true,
                },
            },
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
