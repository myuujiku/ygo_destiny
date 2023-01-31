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

use relm4::prelude::*;

use crate::{components::CollectionPicker, AppInput};
use pages::*;

#[derive(Debug)]
pub enum ViewControllerPage {
    Collection(String),
}

#[derive(Debug)]
pub enum ViewControllerInput {
    AddPage(ViewControllerPage),
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
            append: model.collection_picker.widget(),
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
                let component = match page {
                    ViewControllerPage::Collection(file_name) => CollectionPage::builder()
                        .launch(file_name)
                        .forward(sender.input_sender(), identity),
                };

                root.append(component.widget());
                root.navigate(adw::NavigationDirection::Forward);
            }
            ViewControllerInput::ClosePage => {}
        }
    }
}
