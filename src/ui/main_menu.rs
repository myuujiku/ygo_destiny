use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::glib;
use gtk::CompositeTemplate;

mod imp {
    use super::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/com/myujiku/ygod/ui/main_menu.ui")]
    pub struct MainMenu {}

    #[glib::object_subclass]
    impl ObjectSubclass for MainMenu {
        const NAME: &'static str = "YGOMainMenu";
        type Type = super::MainMenu;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainMenu {}
    impl WidgetImpl for MainMenu {}
    impl BinImpl for MainMenu {}
}

glib::wrapper! {
    pub struct MainMenu(ObjectSubclass<imp::MainMenu>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl MainMenu {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create MainMenu.")
    }
}
