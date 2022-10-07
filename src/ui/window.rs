use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/myujiku/ygod/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub leaflet: TemplateChild<adw::Leaflet>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "YGOWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
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

#[gtk::template_callbacks]
impl Window {
    pub fn new(app: &adw::Application) -> Self {
        let window: Window = glib::Object::new(&[("application", app)])
            .expect("Failed to create instance of Window.");

        window.imp().leaflet.set_visible_child_name("pageidk");

        window
    }

    fn leaflet(&self) -> &gtk::TemplateChild<adw::Leaflet> {
        &self.imp().leaflet
    }

    #[template_callback]
    fn leaflet_back_clicked(window: Window) {
        window.leaflet().navigate(adw::NavigationDirection::Back);
    }
}
