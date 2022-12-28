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

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

use crate::ui::widgets::collection::{collection_options, CollectionOptions};

glib::wrapper! {
    pub struct CollectionCreateWindow(ObjectSubclass<imp::CollectionCreateWindow>)
        @extends adw::PreferencesWindow, adw::Window, gtk::Widget, gtk::Window;
}

impl CollectionCreateWindow {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }

    pub fn from_options(options: CollectionOptions) -> Self {
        let obj = Self::new();
        obj.set_options(options);
        return obj;
    }

    pub fn collect_options(&self) -> CollectionOptions {
        CollectionOptions {
            draft_options: collection_options::DraftOptions {
                rounds: self.draft_rounds_spinner_value(),
                cards: self.draft_cards_spinner_value(),
                follow_sets: self.draft_follow_sets_expander_true(),
                rotate_sets: self.draft_set_rotation_expander_true(),
                rotate_after: self.draft_keep_sets_spinner_value(),
            },
        }
    }

    pub fn set_options(&self, options: CollectionOptions) {
        let imp = self.imp();

        let draft_options = options.draft_options;
        imp.draft_rounds_spinner
            .set_value(draft_options.rounds as f64);
        imp.draft_cards_spinner
            .set_value(draft_options.cards as f64);
        imp.draft_follow_sets_expander
            .set_enable_expansion(draft_options.follow_sets);
        imp.draft_set_rotation_expander
            .set_enable_expansion(draft_options.rotate_sets);
        imp.draft_keep_sets_spinner
            .set_value(draft_options.rotate_after as f64);
    }

    pub fn draft_rounds_spinner_value(&self) -> i32 {
        self.imp().draft_rounds_spinner.value_as_int()
    }

    pub fn draft_cards_spinner_value(&self) -> i32 {
        self.imp().draft_cards_spinner.value_as_int()
    }

    pub fn draft_follow_sets_expander_true(&self) -> bool {
        self.imp().draft_follow_sets_expander.enables_expansion()
    }

    pub fn draft_set_rotation_expander_true(&self) -> bool {
        self.imp().draft_set_rotation_expander.enables_expansion()
    }

    pub fn draft_keep_sets_spinner_value(&self) -> i32 {
        self.imp().draft_keep_sets_spinner.value_as_int()
    }
}
