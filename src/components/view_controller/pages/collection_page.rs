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
                set_title_widget: Some(&adw::WindowTitle::new(&t!("collection_page.title", name = &model.collection.meta_data.name), "")),
                pack_start: &gtk::Button::builder().icon_name("go-previous-symbolic").build(),
            },
        }
    }

    fn init(
        file_name: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
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
