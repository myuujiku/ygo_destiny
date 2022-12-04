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
use gtk::glib;
use once_cell::sync::OnceCell;

use crate::ui::utils::card::load_card;
use crate::ui::widgets::row_split_box::RowSplitBox;

#[derive(Default)]
pub struct DraftBox {
    pub button: gtk::Button,
    pub ids: Vec<u32>,
}

impl DraftBox {
    fn new() -> Self {
        let o = DraftBox::default();

        return o;
    }
}

#[derive(Default)]
pub struct DraftContainer {
    pub layout: RefCell<gtk::Box>,
    pub boxes: RefCell<Vec<RefCell<DraftBox>>>,
    pub number_of_boxes: OnceCell<usize>,
    pub max_selected: OnceCell<usize>,
    pub selected_boxes: RefCell<Vec<usize>>,
}

#[glib::object_subclass]
impl ObjectSubclass for DraftContainer {
    const NAME: &'static str = "YGODraftContainer";
    type Type = super::DraftContainer;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk::BinLayout>();
    }
}

impl ObjectImpl for DraftContainer {
    fn constructed(&self) {
        self.parent_constructed();

        let layout = self.layout.borrow_mut();
        layout.set_orientation(gtk::Orientation::Vertical);
        layout.set_parent(&*self.obj());
        layout.add_css_class("linked");
    }

    fn dispose(&self) {
        self.layout.borrow_mut().unparent();
    }
}
impl WidgetImpl for DraftContainer {}

impl DraftContainer {
    pub fn get_selected_cards(&mut self) -> Vec<u32> {
        let mut selected_cards = Vec::new();

        let boxes = self.boxes.borrow();

        for n in self.selected_boxes.borrow().iter() {
            selected_cards.append(&mut boxes[*n].borrow().ids.to_vec());
        }

        return selected_cards;
    }

    pub fn populate_boxes(&self, card_ids: &Vec<Vec<u32>>) {
        // Clear
        loop {
            let draft_box = self.boxes.borrow_mut().pop();

            if draft_box.is_some() {
                self.layout
                    .borrow_mut()
                    .remove(&draft_box.unwrap().borrow().button);
            } else {
                break;
            }
        }

        let number_of_boxes = *self.number_of_boxes.get().unwrap();

        if card_ids.len() != number_of_boxes {
            panic!(
                "{}",
                format!(
                    "trying to assign {} boxes to a DraftContainer that holds {} boxes.",
                    card_ids.len(),
                    number_of_boxes
                )
            );
        };

        // Repopulate
        for i in 0..number_of_boxes {
            let mut draft_box = DraftBox::new();
            draft_box
                .button
                .set_layout_manager(Some(&gtk::BinLayout::new()));

            self.layout.borrow_mut().append(&draft_box.button);

            let row_split_box = RowSplitBox::new(240, 240, 0, 30);
            row_split_box.set_vexpand(true);
            row_split_box.set_valign(gtk::Align::Center);

            draft_box.button.set_child(Some(&row_split_box));

            draft_box.ids.append(&mut card_ids[i].to_vec());

            for id in &card_ids[i] {
                row_split_box.insert(load_card(*id));
            }

            let selected_boxes = self.selected_boxes.as_ptr();
            let boxes = self.boxes.as_ptr();
            let max_selected = self.max_selected.get().unwrap();

            draft_box.button.connect_clicked(glib::clone!(
                @strong draft_box.button as button, @strong i, @strong selected_boxes, @strong max_selected, @strong boxes =>
                move |_| {
                    let mut_ref: &mut Vec<usize> = unsafe {
                        selected_boxes.as_mut().unwrap()
                    };

                    let index = mut_ref.iter().position(|&x| x == i);

                    // mut_ref contains `i`
                    if index.is_some() {
                        button.remove_css_class("suggested-action");
                        mut_ref.remove(index.unwrap());
                    // mut_ref does not contain `i`
                    } else {
                        button.add_css_class("suggested-action");
                        mut_ref.push(i);

                        if mut_ref.len() > max_selected {
                            let b = &unsafe {
                                boxes.as_ref().unwrap()[mut_ref.remove(0)].borrow()
                            }.button;
                            b.remove_css_class("suggested-action");
                        }
                    }
                }
            ));

            self.boxes.borrow_mut().push(RefCell::new(draft_box));
        }
    }
}
