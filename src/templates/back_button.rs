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

use std::cell::Cell;
use std::fmt::Debug;

use adw::prelude::*;
use relm4::Sender;
use relm4::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for BackButton {
    view! {
        #[name = "button"]
        gtk::Button {
            set_icon_name: "go-previous-symbolic",
        }
    }
}

impl BackButton {
    pub fn set_exit_message<T: Debug + 'static>(&self, sender: &Sender<T>, message: T) {
        let sender = sender.clone();
        let msg_cell = Cell::new(Some(message));

        self.button.connect_clicked(move |_| {
            let msg = msg_cell.replace(None).unwrap();
            sender.send(msg).unwrap();
        });
    }
}
