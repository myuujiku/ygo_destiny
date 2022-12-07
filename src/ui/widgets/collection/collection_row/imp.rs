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

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::{InitializingObject, Signal};
use gtk::{glib, CompositeTemplate};
use once_cell::sync::Lazy;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/collection_row.ui")]
pub struct CollectionRow {
    #[template_child]
    pub star_button: TemplateChild<gtk::Button>,
    pub pinned: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for CollectionRow {
    const NAME: &'static str = "YGOCollectionRow";
    type Type = super::CollectionRow;
    type ParentType = adw::ActionRow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for CollectionRow {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> =
            Lazy::new(|| vec![Signal::builder("pin-action").build()]);
        return SIGNALS.as_ref();
    }

    fn constructed(&self) {
        self.parent_constructed();

        let self_ptr = self as *const Self;

        // Change star_button icon and pinned state on clicked
        self.star_button.connect_clicked(move |_| unsafe {
            let ptr_ref = self_ptr.as_ref().unwrap();
            let new_val = !ptr_ref.pinned.get();

            ptr_ref.pinned.set(new_val);
            ptr_ref.obj().emit_by_name::<()>("pin-action", &[]);

            if new_val {
                ptr_ref.star_button.set_icon_name("starred-symbolic");
            } else {
                ptr_ref.star_button.set_icon_name("non-starred-symbolic");
            }
        });

        // Modify button appearance if the row is not pinned
        let row_motion_controller = gtk::EventControllerMotion::new();
        self.obj().add_controller(&row_motion_controller);

        // Show non-starred icon if mouse cursor is inside the collection_row
        row_motion_controller.connect_enter(move |_, _, _| unsafe {
            let ptr_ref = self_ptr.as_ref().unwrap();
            if !ptr_ref.pinned.get() {
                ptr_ref.star_button.set_icon_name("non-starred-symbolic")
            }
        });

        // Hide non-starred icon if mouse cursor is outside the collection row
        row_motion_controller.connect_leave(move |_| unsafe {
            let ptr_ref = self_ptr.as_ref().unwrap();
            if !ptr_ref.pinned.get() {
                ptr_ref.star_button.set_icon_name("");
            }
        });
    }

    fn dispose(&self) {
        // Unparent all direct children
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}

impl WidgetImpl for CollectionRow {}
impl ActionRowImpl for CollectionRow {}
impl ListBoxRowImpl for CollectionRow {}
impl PreferencesRowImpl for CollectionRow {}
