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

use crate::templates::{BackButton, StatusPage};
use super::DraftSettingsPageInput;
use super::DraftBox;

pub struct DraftBoxSettings {
    draft_box: Option<DraftBox>,
}

#[relm4::component(pub)]
impl SimpleComponent for DraftBoxSettings {
    type Init = Option<DraftBox>;
    type Input = ();
    type Output = DraftSettingsPageInput;
    type Widgets = DraftBoxSettingsWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: Orientation::Vertical,

            adw::HeaderBar {
                set_title_widget: Some(&adw::WindowTitle::new(
                    "Draft Type",
                    "",
                )),
                #[template]
                pack_start = &BackButton {
                    set_exit_message: (sender.output_sender(), DraftSettingsPageInput::CloseDraftBox),
                },
            },
            #[name = "leaflet"]
            adw::Leaflet {
                set_can_unfold: false,
                set_transition_type: adw::LeafletTransitionType::Slide,

                gtk::Box {
                    set_orientation: Orientation::Vertical,

                    gtk::Box {
                        set_orientation: Orientation::Horizontal,

                        #[name = "carousel"]
                        adw::Carousel {
                            set_orientation: Orientation::Vertical,
                            set_hexpand: true,
                            set_vexpand: true,

                            #[template]
                            StatusPage {
                                set_title: "Choice Draft",
                                set_description: "You have no choice (currently).",

                                #[wrap(Some)]
                                set_child = &gtk::Button {
                                    add_css_class: "suggested-action",
                                    add_css_class: "pill",
                                    set_label: "Configure",
                                    set_halign: Align::Center,
                                },
                            },
                            #[template]
                            StatusPage {
                                set_title: "Standard Draft",
                            },
                            #[template]
                            StatusPage {
                                set_title: "Battle Pack Draft",
                            },
                        },
                        adw::CarouselIndicatorLines {
                            set_carousel: Some(&carousel),
                            set_orientation: Orientation::Vertical,
                        },
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { draft_box: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
