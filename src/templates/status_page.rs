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
use gtk::{Align, Justification, Orientation, Widget, pango::WrapMode};
use relm4::prelude::*;

/// Simpler recreation of the Libadwaita `StatusPage`.
#[relm4::widget_template(pub)]
impl WidgetTemplate for StatusPage {
    view! {
        adw::Clamp {
            set_vexpand: true,

            gtk::Box {
                set_orientation: Orientation::Vertical,
                set_valign: Align::Center,
                set_spacing: 12,

                #[name = "title"]
                gtk::Label {
                    add_css_class: "title",
                    add_css_class: "title-1",
                    set_visible: false,
                    set_wrap: true,
                    set_wrap_mode: WrapMode::WordChar,
                    set_justify: Justification::Center,
                },

                #[name = "desc"]
                gtk::Label {
                    set_visible: false,
                    set_wrap: true,
                    set_wrap_mode: WrapMode::WordChar,
                    set_justify: Justification::Center,
                },

                #[name = "spacer"]
                gtk::Separator {
                    set_visible: false,
                    add_css_class: "spacer",
                    set_orientation: Orientation::Vertical,
                },

                #[name = "bin"]
                adw::Bin {
                    set_visible: false,
                },
            }
        }
    }
}

impl StatusPage {
    pub fn set_title(&self, text: &str) {
        self.title.set_visible(!text.is_empty());
        self.title.set_label(text);
    }

    pub fn set_description(&self, text: &str) {
        self.desc.set_visible(!text.is_empty());
        self.desc.set_label(text);
    }

    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        let is_some = child.is_some();
        self.bin.set_visible(is_some);
        self.spacer.set_visible(is_some);

        self.bin.set_child(child);
    }
}
