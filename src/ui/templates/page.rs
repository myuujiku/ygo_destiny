use std::{
    cell::Cell,
    fmt::Debug,
};

use adw::prelude::*;
use relm4::Sender;
use relm4::prelude::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for Page {
    view! {
        #[name = "content"]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[name = "header"]
            adw::HeaderBar {
                #[name = "back_button"]
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous-symbolic",
                }
            }
        }
    }
}

impl Page {
    pub fn set_back_button_target<T: Debug + 'static>(&self, sender: &Sender<T>, message: T) {
        let sender = sender.clone();
        let msg_cell = Cell::new(Some(message));

        self.back_button.connect_clicked(move |_| {
            let msg = msg_cell.replace(None).unwrap();
            sender.send(msg).unwrap();
        });
    }
}
