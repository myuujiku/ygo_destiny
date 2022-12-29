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

mod imp;

use std::fs;
use std::sync::Mutex;
use std::thread;

use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::{Continue, MainContext, PRIORITY_DEFAULT};
use gtk::{gio, glib};
use std::cell::Cell;

use crate::logic::{
    ext_data::image_dl,
    user_data::ProgressiveCollection,
    utils::{http, PATHS},
};
use crate::ui::pages::{EditCollectionPage, UpdatePage};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl Window {
    pub fn new<P: glib::IsA<adw::Application>>(app: &P) -> Self {
        let window: Window = glib::Object::new(&[("application", app)]);

        return window;
    }

    // TODO: Remove this from here and add it to the update page
    pub fn obj(&self) -> glib::BorrowedObject<Self> {
        self.imp().obj()
    }

    pub fn open_collection(&self, name: &String) {
        let edit_page = EditCollectionPage::new(name);

        let leaflet = self.get_leaflet().get();
        leaflet.append(&edit_page);
        leaflet.navigate(adw::NavigationDirection::Forward);

        edit_page
            .imp()
            .back_button
            .connect_clicked(glib::clone!(@weak leaflet => move |_| {
                leaflet.remove(&leaflet.visible_child().unwrap());
            }));
    }

    pub fn update_collections(&self) {
        self.imp().collection_list.imp().update_model();
    }

    pub fn get_new_collection(&self) -> ProgressiveCollection {
        let imp = self.imp();
        imp.collection_list.imp().popover.popdown();

        ProgressiveCollection::builder()
            .meta_data(imp.collection_list.new_collection_meta_data())
            .draft_settings(imp.collection_options.borrow().as_draft_settings())
            .build()
    }

    pub fn show_update_notification(&self) {
        // *self.get_update_version().lock().unwrap() = http::update_version();
        let update_version = http::update_version();
        if update_version.is_some() {
            *self.get_update_version().lock().unwrap() = update_version.unwrap();

            let toast_overlay = self.get_toast_overlay();

            let toast = adw::Toast::new("New update available.");
            toast.set_button_label(Some("Update"));
            toast.set_action_name(Some("win.update_data"));

            toast_overlay.add_toast(&toast);
        }
    }

    pub fn update(&self, finished_sender: glib::Sender<()>) {
        let leaflet = self.get_leaflet();
        let update_page = UpdatePage::new();

        leaflet.append(&update_page);
        leaflet.navigate(adw::NavigationDirection::Forward);

        let update_version = self.get_update_version().lock().unwrap().clone();

        // Do the update in a different thread
        let (progress_sender, progress_receiver) = MainContext::channel(PRIORITY_DEFAULT);

        thread::spawn(move || {
            let update_status = http::update();

            match update_status {
                http::UpdateStatus::Complete => {
                    fs::write(&PATHS.ext_data.version, update_version).unwrap();
                    image_dl::cards::download_missing_cards(
                        image_dl::cards::ImageType::Big,
                        progress_sender,
                    );
                }
                x => println!("{:#?}", x),
            }
            finished_sender
                .send(())
                .expect("Could not send through channel");
        });

        let first_progress_update = Cell::new(true);

        progress_receiver.attach(
            None,
            glib::clone!(@weak update_page, @strong first_progress_update => @default-return Continue(false),
                move |args: (f64, String)| {
                    // Switch to progress bar
                    if first_progress_update.get() {
                        update_page.get_leaflet().set_visible_child_name("progress_page");
                        first_progress_update.set(false);
                    }

                    let (frac, text) = args;
                    update_page.imp().progress_bar.set_fraction(frac);
                    update_page.imp().label.set_label(&text);
                    Continue(true)
                }
            ),
        );
    }

    pub fn get_leaflet(&self) -> &gtk::TemplateChild<adw::Leaflet> {
        return &self.imp().leaflet;
    }

    fn get_toast_overlay(&self) -> &gtk::TemplateChild<adw::ToastOverlay> {
        return &self.imp().toast_overlay;
    }

    fn get_update_version(&self) -> &Mutex<String> {
        return &self.imp().update_version;
    }
}
