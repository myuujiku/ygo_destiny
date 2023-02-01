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
use gtk::{glib, Orientation};
use relm4::prelude::*;
use rust_i18n::t;
use ygod_core::user_data::Collection;

use crate::components::ViewControllerInput;

pub struct CreateCollectionPage {
    pub collection: Collection,
    go_back_button: gtk::Button,
}

#[relm4::component(pub)]
impl SimpleComponent for CreateCollectionPage {
    type Init = ();
    type Input = ();
    type Output = ViewControllerInput;
    type Widgets = CreateCollectionPageWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: Orientation::Vertical,

            adw::HeaderBar {
                set_title_widget: Some(&adw::WindowTitle::new(
                    &t!("create_collection_page.title"),
                    "",
                )),
                pack_start: &model.go_back_button,
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let collection = Collection::default();

        let go_back_button = gtk::Button::builder()
            .icon_name("go-previous-symbolic")
            .build();
        go_back_button.connect_clicked(glib::clone!(@strong sender => move |_| {
            sender.output(ViewControllerInput::ClosePage).unwrap();
        }));

        let model = Self {
            collection,
            go_back_button,
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
