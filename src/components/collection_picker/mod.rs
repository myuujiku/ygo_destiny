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

use relm4::prelude::*;

pub struct CollectionPicker;

#[relm4::component(pub)]
impl SimpleComponent for CollectionPicker {
    type Init = ();
    type Input = ();
    type Output = ();
    type Widgets = CollectionPickerWidgets;

    view! {
        #[root]
        gtk::Box::new(gtk::Orientation::Vertical, 6) {}
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
