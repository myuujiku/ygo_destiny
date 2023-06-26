use adw::prelude::*;
use relm4::prelude::*;

use crate::data::images::load_card;
use crate::ui::widgets::RowSplitBox;

#[derive(Debug)]
pub enum DraftBoxInput {
    NotifyClicked,
}

#[derive(Debug)]
pub enum DraftBoxOutput {
    Clicked(usize),
}

#[derive(Debug)]
pub struct DraftBoxParams {
    cards: Vec<u32>,
    id: usize,
}

impl DraftBoxParams {
    pub fn new(cards: Vec<u32>, id: usize) -> Self {
        Self { cards, id }
    }
}

#[derive(Debug)]
pub struct DraftBox {
    pub cards: Vec<u32>,
    id: usize,
}

#[relm4::component(pub)]
impl Component for DraftBox {
    type Init = DraftBoxParams;
    type Input = DraftBoxInput;
    type Output = DraftBoxOutput;
    type CommandOutput = ();
    type Widgets = DraftBoxWidgets;

    view! {
        #[root]
        gtk::Button {
            set_layout_manager: Some(gtk::BinLayout::new()),
            connect_clicked[sender] => move |_| sender.input(DraftBoxInput::NotifyClicked),

            #[name = "row_split_box"]
            #[wrap(Some)]
            set_child = &RowSplitBox::new(240, 240, 0, 30) {
                set_vexpand: true,
                set_valign: gtk::Align::Center,
            }
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            cards: params.cards,
            id: params.id,
        };

        let widgets = view_output!();

        for id in &model.cards {
            widgets.row_split_box.insert(load_card(*id));
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match input {
            DraftBoxInput::NotifyClicked => {
                sender.output(DraftBoxOutput::Clicked(self.id)).unwrap();
            }
        }
    }
}
