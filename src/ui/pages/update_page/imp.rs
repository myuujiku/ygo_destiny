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
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/myujiku/ygo_destiny/templates/update_page.ui")]
pub struct UpdatePage {
    #[template_child]
    pub progress_bar: TemplateChild<gtk::ProgressBar>,
}

#[glib::object_subclass]
impl ObjectSubclass for UpdatePage {
    const NAME: &'static str = "YGOUpdatePage";
    type Type = super::UpdatePage;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for UpdatePage {
    fn dispose(&self) {
        self.progress_bar.unparent();
    }
}

impl WidgetImpl for UpdatePage {}
impl BinImpl for UpdatePage {}
