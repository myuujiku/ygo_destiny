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

use std::convert::identity;

use adw::prelude::*;
use relm4::prelude::*;

use crate::{
    components::{ViewController, ViewControllerEvent},
    templates::ContentBox,
};

#[derive(Debug)]
pub enum AppEvent {
    ChangView(ViewControllerEvent),
}

pub struct App {
    view_controller: Controller<ViewController>,
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppEvent;
    type Output = ();

    view! {
        adw::Window {
            set_width_request: 640,
            set_height_request: 480,

            #[template]
            ContentBox {
                adw::HeaderBar {
                    set_title_widget: Some(&adw::WindowTitle::new("YGO Destiny", "")),
                    pack_start: &gtk::Button::builder().icon_name("open-menu-symbolic").build(),
                },
                model.view_controller.widget(),
            },
        }
    }
    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            view_controller: ViewController::builder()
                .launch(())
                .forward(sender.input_sender(), identity),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>) {
        match input  {
            AppEvent::ChangView(_message) => (),
        }
    }
}
