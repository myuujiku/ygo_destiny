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

mod draft_box_settings;

use std::convert::identity;
use std::time::Duration;

use adw::{prelude::*, NavigationDirection};
use gtk::{glib, Align, Orientation, PolicyType};
use relm4::prelude::*;
use rust_i18n::t;
use ygod_core::user_data::DraftBox;

use crate::components::ViewControllerInput;
use crate::templates::BackButton;
use draft_box_settings::DraftBoxSettings;

#[derive(Debug)]
pub enum DraftSettingsPageInput {
    OpenDraftBox(Option<DraftBox>),
    CloseDraftBox,
}

pub struct DraftSettingsPage {
    // settings: Vec<(DraftSettings, usize)>,
}

#[relm4::component(pub)]
impl Component for DraftSettingsPage {
    type Init = ();
    type Input = DraftSettingsPageInput;
    type Output = ViewControllerInput;
    type CommandOutput = ();
    type Widgets = DraftSettingsPageWidgets;

    view! {
        #[root]

        adw::Leaflet {
            set_can_unfold: false,
            set_transition_type: adw::LeafletTransitionType::Slide,

            gtk::Box {
                set_orientation: Orientation::Vertical,

                adw::HeaderBar {
                    set_title_widget: Some(&adw::WindowTitle::new("Draft Settings", "")),
                    #[template]
                    pack_start = &BackButton {
                        set_exit_message: (sender.output_sender(), ViewControllerInput::ClosePage),
                    },
                },
                gtk::ScrolledWindow {
                    set_min_content_height: 200,
                    set_hscrollbar_policy: PolicyType::Never,
                    set_vexpand: true,

                    adw::Clamp {
                        set_orientation: Orientation::Horizontal,
                        set_maximum_size: 800,

                        gtk::Box::new(Orientation::Vertical, 6) {
                            set_hexpand: true,
                            set_valign: Align::Center,
                            set_margin_all: 6,

                            adw::PreferencesGroup {
                                set_title: &t!("pages.draft_settings.draft_boxes"),
                                #[wrap(Some)]
                                set_header_suffix = &gtk::Button {
                                    add_css_class: "flat",
                                    set_icon_name: "list-add",

                                    connect_clicked => DraftSettingsPageInput::OpenDraftBox(None),
                                },
                            },
                        }
                    }
                },
                gtk::Box {
                    set_visible: false,
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

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match input {
            DraftSettingsPageInput::OpenDraftBox(settings) => {
                let mut controller = DraftBoxSettings::builder()
                    .launch(settings)
                    .forward(sender.input_sender(), identity);
                controller.detach_runtime();
                root.append(controller.widget());
                root.navigate(NavigationDirection::Forward);
            }
            DraftSettingsPageInput::CloseDraftBox => {
                let main_context = glib::MainContext::default();
                main_context.spawn_local(glib::clone!(@strong root => async move {
                    // Get current page
                    let to_remove = root.visible_child().unwrap();
                    // Initiate navigation to the previous page
                    root.navigate(NavigationDirection::Back);

                    // Wait for navigation animation to complete
                    let duration = Duration::from_millis(root.mode_transition_duration() as u64);
                    glib::timeout_future(duration).await;

                    // Remove previously displayed page
                    root.remove(&to_remove);
                }));
            }
        }
    }
}
