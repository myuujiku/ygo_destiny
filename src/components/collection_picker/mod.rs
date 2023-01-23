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

use crate::AppInput;

pub struct CollectionPicker;

#[relm4::component(pub)]
impl SimpleComponent for CollectionPicker {
    type Init = ();
    type Input = ();
    type Output = AppInput;
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
                    adw::StatusPage {
                        set_icon_name: Some("preferences-desktop-screensaver"),
                        set_title: "No Collections Found",
                        set_description: Some("Create a new collection using the plus button."),
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
        let model = Self;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
