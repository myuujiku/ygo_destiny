mod imp;

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use relm4::gtk;

glib::wrapper! {
    pub struct RowSplitBox(ObjectSubclass<imp::RowSplitBox>)
        @implements gtk::Widget, gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RowSplitBox {
    pub fn new(cell_width: i32, cell_height: i32, h_spacing: i32, v_spacing: i32) -> Self {
        let object: Self = glib::Object::builder().build();
        let imp = object.imp();

        imp.cell_width.set(cell_width).unwrap();
        imp.cell_height.set(cell_height).unwrap();
        imp.h_spacing.set(h_spacing).unwrap();
        imp.v_spacing.set(v_spacing).unwrap();

        object
    }

    pub fn insert(&self, widget: gtk::Image) {
        widget.set_parent(self);
        self.imp().children.borrow_mut().push(widget);
    }
}
