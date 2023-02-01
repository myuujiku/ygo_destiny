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
use gtk::glib;
use relm4::prelude::*;
use ygod_core::user_data::MetaData;

#[derive(Debug, PartialEq)]
pub struct TrackedBool {
    original_value: bool,
    current_value: bool,
}

impl TrackedBool {
    pub fn new(value: bool) -> Self {
        Self {
            original_value: value,
            current_value: value,
        }
    }

    pub fn get(&self) -> bool {
        self.current_value
    }

    pub fn toggle(&mut self) -> bool {
        let new_value = !self.current_value;
        self.set(new_value);

        new_value
    }

    pub fn set(&mut self, new_value: bool) {
        self.current_value = new_value;
    }

    pub fn has_changed(&self) -> bool {
        self.current_value != self.original_value
    }
}

#[derive(Debug)]
pub struct CollectionData {
    pub file_name: String,
    pub meta_data: MetaData,
}

impl CollectionData {
    pub fn new(file_name: String, meta_data: MetaData) -> Self {
        Self {
            file_name,
            meta_data,
        }
    }
}

#[derive(Debug)]
pub enum CollectionEntryInput {
    TogglePinned,
    SetVisible(bool),
    CursorEntered,
    CursorLeft,
    Open,
}

#[derive(Debug)]
pub enum CollectionEntryOutput {
    SortUp(DynamicIndex),
    SortDown(DynamicIndex),
    FilterBy(String),
    OpenCollection(String),
    SaveChanges,
}

#[derive(Debug)]
pub struct CollectionEntry {
    pub file: String,
    pub name: String,
    pub description: String,
    pub last_modified: String,
    pub pinned: TrackedBool,
    index: DynamicIndex,
}

#[relm4::factory(pub)]
impl FactoryComponent for CollectionEntry {
    type Init = CollectionData;
    type Input = CollectionEntryInput;
    type Output = CollectionEntryOutput;
    type CommandOutput = ();
    type Widgets = CollectionEntryWidgets;
    type ParentInput = CollectionEntryOutput;
    type ParentWidget = gtk::ListBox;

    view! {
        #[name = "root"]
        #[root]
        adw::ActionRow {
            set_selectable: false,
            set_title: &self.name,
            set_subtitle: &self.description,
            set_activatable_widget: Some(&gtk::Label::new(None)),
            connect_activated => CollectionEntryInput::Open,
            add_controller: &{
                let controller = gtk::EventControllerMotion::new();
                controller.connect_enter(glib::clone!(@strong sender => move |_, _, _| {
                    sender.input(CollectionEntryInput::CursorEntered)
                }));
                controller.connect_leave(glib::clone!(@strong sender => move |_| {
                    sender.input(CollectionEntryInput::CursorLeft)
                }));

                controller
            },
            add_suffix: star_button = &gtk::Button {
                set_icon_name?: match self.pinned.get() {
                    true => Some("starred-symbolic"),
                    false => None,
                },
                set_valign: gtk::Align::Center,
                connect_clicked => CollectionEntryInput::TogglePinned,
                add_css_class: "flat",
            },
        }
    }

    fn output_to_parent_input(output: Self::Output) -> Option<Self::Output> {
        Some(output)
    }

    fn init_model(value: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            file: value.file_name,
            name: value.meta_data.name,
            description: value.meta_data.description,
            last_modified: value.meta_data.last_changed,
            pinned: TrackedBool::new(value.meta_data.pinned),
            index: index.clone(),
        }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        input: Self::Input,
        sender: FactorySender<Self>,
    ) {
        match input {
            CollectionEntryInput::TogglePinned => match self.pinned.toggle() {
                true => {
                    widgets.star_button.set_icon_name("starred-symbolic");
                    sender.output(CollectionEntryOutput::SortUp(self.index.clone()));
                }
                false => {
                    widgets.star_button.set_icon_name("non-starred-symbolic");
                    sender.output(CollectionEntryOutput::SortDown(self.index.clone()));
                }
            },
            CollectionEntryInput::SetVisible(value) => widgets.root.set_visible(value),
            CollectionEntryInput::CursorEntered => {
                if !self.pinned.get() {
                    widgets.star_button.set_icon_name("non-starred-symbolic");
                }
            }
            CollectionEntryInput::CursorLeft => {
                if !self.pinned.get() {
                    widgets.star_button.set_icon_name("");
                }
            }
            CollectionEntryInput::Open => {
                sender.output(CollectionEntryOutput::OpenCollection(self.file.clone()));
            }
        }
    }
}
