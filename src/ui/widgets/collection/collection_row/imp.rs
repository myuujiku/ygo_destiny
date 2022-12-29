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

use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::{InitializingObject, Signal};
use glib::{ParamSpec, ParamSpecBoolean, ParamSpecObject, Value};
use gtk::{glib, CompositeTemplate};
use once_cell::sync::Lazy;

use crate::ui::widgets::collection::CollectionData;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/collection_row.ui")]
pub struct CollectionRow {
    #[template_child]
    pub star_button: TemplateChild<gtk::Button>,
    pub data: RefCell<Option<CollectionData>>,
    pub pinned: RefCell<bool>,
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

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecObject::builder::<CollectionData>("data")
                    .construct_only()
                    .build(),
                ParamSpecBoolean::builder("pinned").build(),
            ]
        });
        return PROPERTIES.as_ref();
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "data" => {
                let data = value.get().unwrap();
                self.data.replace(data);
            }
            "pinned" => {
                let pinned = value.get().unwrap();
                self.pinned.replace(pinned);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "data" => self.data.borrow().to_value(),
            "pinned" => self.pinned.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        let item = self.data.borrow().as_ref().cloned().unwrap();

        item.bind_property("star", obj.as_ref(), "pinned")
            .sync_create()
            .bidirectional()
            .build();

        obj.set_title(&item.property::<String>("name"));
        obj.set_subtitle(&item.property::<String>("desc"));

        if self.pinned.borrow().clone() {
            self.star_button.set_icon_name("starred-symbolic");
        }

        self.star_button
            .connect_clicked(glib::clone!(@weak item, @weak obj => move |btn| {
                let new_val = !item.property::<bool>("star");
                item.set_property("star", new_val);

                if new_val {
                    btn.set_icon_name("starred-symbolic");
                } else {
                    btn.set_icon_name("non-starred-symbolic");
                }

                obj.emit_by_name::<()>("pin-action", &[]);
            }));

        // Modify button appearance if the row is not pinned
        let row_motion_controller = gtk::EventControllerMotion::new();
        obj.add_controller(&row_motion_controller);

        // Show non-starred icon if mouse cursor is inside the collection_row
        row_motion_controller.connect_enter(
            glib::clone!(@weak item, @strong self.star_button as btn => move |_, _, _| {
                if !item.property::<bool>("star") {
                    btn.set_icon_name("non-starred-symbolic")
                }
            }),
        );

        // Hide non-starred icon if mouse cursor is outside the collection row
        row_motion_controller.connect_leave(
            glib::clone!(@weak item, @strong self.star_button as btn => move |_| {
                if !item.property::<bool>("star") {
                    btn.set_icon_name("")
                }
            }),
        );
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
