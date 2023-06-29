use std::cell::RefCell;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk::Rectangle, glib};
use once_cell::sync::OnceCell;
use relm4::gtk;

#[derive(Default)]
pub struct RowSplitBox {
    pub children: RefCell<Vec<gtk::Widget>>,
    pub cell_width: OnceCell<i32>,
    pub cell_height: OnceCell<i32>,
    pub h_spacing: OnceCell<i32>,
    pub v_spacing: OnceCell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowSplitBox {
    const NAME: &'static str = "YGODRowSplitBox";
    type Type = super::RowSplitBox;
    type ParentType = gtk::Widget;
}

impl ObjectImpl for RowSplitBox {
    fn constructed(&self) {
        self.parent_constructed();
    }

    fn dispose(&self) {
        for child in self.children.borrow_mut().iter() {
            child.unparent();
        }
    }
}

impl WidgetImpl for RowSplitBox {
    fn size_allocate(&self, width: i32, _height: i32, _baseline: i32) {
        self.update_children(width);
    }
}

impl RowSplitBox {
    fn update_children(&self, allocated_width: i32) {
        let cell_width: i32 = *self.cell_width.get().unwrap();
        let cell_height: i32 = *self.cell_height.get().unwrap();
        let h_spacing: i32 = *self.h_spacing.get().unwrap();
        let v_spacing: i32 = *self.v_spacing.get().unwrap();

        let child_count = self.children.borrow().len() as i32;

        let padded_width = cell_width + h_spacing;
        let padded_height = cell_height + v_spacing;

        let column_count = 1 + (allocated_width - cell_width) / (cell_width + h_spacing);

        let needed_space = column_count * cell_width + (column_count - 1) * h_spacing;
        let unneeded_space = (allocated_width - needed_space) / 2;
        let full_rows = child_count / column_count;
        let last_row_elements = child_count % column_count;

        for row in 0..full_rows {
            for element in 0..column_count {
                let alloc = Rectangle::new(
                    unneeded_space + element * padded_width,
                    padded_height * row,
                    cell_width,
                    cell_height,
                );

                self.children.borrow_mut()[(row * column_count + element) as usize]
                    .size_allocate(&alloc, -1);
            }
        }

        let offset = allocated_width / 2
            - (last_row_elements * cell_width + (last_row_elements - 1) * h_spacing) / 2;

        for element in 0..last_row_elements {
            let alloc = Rectangle::new(
                offset + element * padded_width,
                padded_height * full_rows,
                cell_width,
                cell_height,
            );

            self.children.borrow_mut()[(full_rows * column_count + element) as usize]
                .size_allocate(&alloc, -1);
        }

        let target_height = cell_height
            + padded_height * (full_rows - 1)
            + if last_row_elements > 0 {
                padded_height
            } else {
                0
            };

        self.obj().set_size_request(0, target_height);
    }
}
