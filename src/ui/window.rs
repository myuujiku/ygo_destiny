use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::CompositeTemplate;

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/myujiku/ygod/ui/window.ui")]
    pub struct Window {}

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "YGOWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create instance of Window.")
    }
}
