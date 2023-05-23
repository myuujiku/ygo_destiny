use adw::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub enum AppInput {}

pub struct App;

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        adw::Window {
            set_width_request: 640,
            set_height_request: 480,
            add_css_class: "devel",
        }
    }

    fn init(
        _params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>) {
        match input {}
    }
}
