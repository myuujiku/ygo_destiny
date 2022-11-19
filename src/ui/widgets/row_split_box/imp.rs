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
use std::sync::Mutex;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, glib};

#[derive(Default)]
pub struct RowSplitBox {
    pub children: Mutex<Vec<RefCell<gtk::Image>>>,
    pub cell_width: Mutex<i32>,
    pub cell_height: Mutex<i32>,
    pub h_spacing: Mutex<i32>,
    pub v_spacing: Mutex<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowSplitBox {
    const NAME: &'static str = "YGORowSplitBox";
    type Type = super::RowSplitBox;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for RowSplitBox {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {
        for child in self.children.lock().unwrap().iter() {
            child.borrow().unparent();
        }
    }
}

impl WidgetImpl for RowSplitBox {
    fn size_allocate(&self, allocated_width: i32, _allocated_height: i32, _allocated_baseline: i32) {
        let child_count = self.children.lock().unwrap().len() as i32;

        let cell_width = *self.cell_width.lock().unwrap();
        let cell_height = *self.cell_height.lock().unwrap();
        let h_spacing = *self.h_spacing.lock().unwrap();
        let v_spacing = *self.v_spacing.lock().unwrap();

        let padded_width = cell_width + h_spacing;
        let padded_height = cell_height + v_spacing;

        let column_count = 1 + (allocated_width - cell_width) / (cell_width + h_spacing);

        let necessary_space = column_count * cell_width + (column_count - 1) * h_spacing;
        let redundant_space = (allocated_width - necessary_space) / 2;
        let full_rows = child_count / column_count;
        let last_row_elements = child_count % column_count;

        for row in 0..full_rows {
            for element in 0..column_count {
                let alloc = gdk::Rectangle::new(
                    redundant_space + element * padded_width,
                    padded_height * row,
                    cell_width,
                    cell_height,
                );

                self.children.lock().unwrap()[(row * column_count + element) as usize].borrow().size_allocate(&alloc, -1);
            }
        }

        let offset = allocated_width / 2 - (last_row_elements * cell_width + (last_row_elements - 1) * h_spacing) / 2;
        for element in 0..last_row_elements {
            let alloc = gdk::Rectangle::new(
                offset + element * padded_width,
                //(column_count - last_row_elements) * (padded_width) + element * padded_width,
                padded_height * full_rows,
                cell_width,
                cell_height,
            );

            self.children.lock().unwrap()[(full_rows * column_count + element) as usize].borrow().size_allocate(&alloc, -1);
        }
    }
}
