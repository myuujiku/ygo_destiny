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

mod pages;

use std::convert::identity;
use std::time::Duration;

use adw::{prelude::*, NavigationDirection};
use gtk::glib;
use relm4::prelude::*;

use crate::{components::CollectionPicker, AppInput};
use pages::*;

#[derive(Debug)]
pub enum ViewControllerPage {
    Collection(String),
    CreateCollection,
    DraftSettings/* (draft_settings) */,
}

#[derive(Debug)]
pub enum ViewControllerInput {
    AddPage(ViewControllerPage),
    ReplacePage(ViewControllerPage),
    ClosePage,
}

pub struct ViewController {
    collection_picker: Controller<CollectionPicker>,
}

#[relm4::component(pub)]
impl Component for ViewController {
    type Init = ();
    type Input = ViewControllerInput;
    type Output = AppInput;
    type CommandOutput = ();
    type Widgets = ViewControllerWidgets;

    view! {
        #[root]
        adw::Leaflet {
            set_can_unfold: false,
            set_transition_type: adw::LeafletTransitionType::Slide,

            append = model.collection_picker.widget(),
        }
    }

    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            collection_picker: CollectionPicker::builder()
                .launch(())
                .forward(sender.input_sender(), identity),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match input {
            ViewControllerInput::AddPage(page) => {
                match page {
                    ViewControllerPage::Collection(file_name) => {
                        let mut controller = CollectionPage::builder()
                            .launch(file_name)
                            .forward(sender.input_sender(), identity);
                        controller.detach_runtime();
                        root.append(controller.widget());
                    }
                    ViewControllerPage::CreateCollection => {
                        let mut controller = CreateCollectionPage::builder()
                            .launch(())
                            .forward(sender.input_sender(), identity);
                        controller.detach_runtime();
                        root.append(controller.widget());
                    }
                    ViewControllerPage::DraftSettings => {
                        let mut controller = DraftSettingsPage::builder()
                            .launch(())
                            .forward(sender.input_sender(), identity);
                        controller.detach_runtime();
                        root.append(controller.widget());
                    }
                };

                root.navigate(NavigationDirection::Forward);
            }
            ViewControllerInput::ReplacePage(page) => {
                let main_context = glib::MainContext::default();
                main_context.spawn_local(glib::clone!(@strong root => async move {
                    // Get current page
                    let to_remove = root.visible_child().unwrap();

                    // Add page
                    sender.input(ViewControllerInput::AddPage(page));

                    // Wait for navigation animation to complete
                    let duration = Duration::from_millis(root.mode_transition_duration() as u64);
                    glib::timeout_future(duration).await;

                    // Remove previously displayed page
                    root.remove(&to_remove);
                }));
            }
            ViewControllerInput::ClosePage => {
                // Check if the page that will be displayed next is the root page
                if root.pages().n_items() == 2 {
                    // Re-initialise the collection picker to reflect changes and newly added collections
                    self.collection_picker = CollectionPicker::builder()
                        .launch(())
                        .forward(sender.input_sender(), identity);
                    root.remove(&root.adjacent_child(NavigationDirection::Back).unwrap());
                    root.prepend(self.collection_picker.widget());
                }

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
