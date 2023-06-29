mod draft_box;

use std::collections::VecDeque;

use adw::prelude::*;
use gtk::Orientation;
use relm4::prelude::*;

use draft_box::*;

#[derive(Debug)]
pub enum DraftContainerInput {
    Populate(Vec<Vec<u32>>),
    BoxClicked(usize),
    RequestSelected,
}

#[derive(Debug)]
pub enum DraftContainerOutput {
    SelectionComplete(Vec<u32>),
    SelectionValid(bool),
}

#[derive(Debug)]
pub struct DraftContainerParams {
    pub number_of_boxes: usize,
    pub max_selected: usize,
}

impl DraftContainerParams {
    pub fn new(number_of_boxes: usize, max_selected: usize) -> Self {
        Self {
            number_of_boxes,
            max_selected,
        }
    }
}

#[derive(Debug)]
pub struct DraftContainer {
    number_of_boxes: usize,
    boxes: Vec<Controller<DraftBox>>,
    max_selected: usize,
    selected_boxes: VecDeque<usize>,
    selection_valid: bool,
}

#[relm4::component(pub)]
impl Component for DraftContainer {
    type Init = DraftContainerParams;
    type Input = DraftContainerInput;
    type Output = DraftContainerOutput;
    type CommandOutput = ();
    type Widgets = DraftContainerWidgets;

    view! {
        #[root]
        gtk::Box::new(Orientation::Vertical, 6) {
            add_css_class: "linked",
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            selected_boxes: VecDeque::new(),
            boxes: Vec::new(),
            max_selected: params.max_selected,
            number_of_boxes: params.number_of_boxes,
            selection_valid: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match input {
            DraftContainerInput::Populate(mut cards) => {
                while let Some(draft_box) = self.boxes.pop() {
                    root.remove(draft_box.widget());
                }

                if cards.len() != self.number_of_boxes {
                    panic!(
                        "Trying to assign {} boxes to a DraftContainer that holds {} boxes.",
                        cards.len(),
                        self.number_of_boxes
                    );
                }

                for i in 0..self.number_of_boxes {
                    let draft_box = DraftBox::builder()
                        .launch(DraftBoxParams::new(
                            cards.pop().expect("Size already checked."),
                            i,
                        ))
                        .forward(sender.input_sender(), |msg| match msg {
                            DraftBoxOutput::Clicked(id) => DraftContainerInput::BoxClicked(id),
                        });

                    root.append(draft_box.widget());
                    self.boxes.push(draft_box);
                }
            }
            DraftContainerInput::BoxClicked(id) => {
                if let Some(selected_pos) = self.selected_boxes.iter().position(|&x| x == id) {
                    self.boxes[id].widget().remove_css_class("suggested-action");
                    self.selected_boxes.remove(selected_pos);
                } else {
                    self.boxes[id].widget().add_css_class("suggested-action");
                    self.selected_boxes.push_back(id);

                    if self.selected_boxes.len() > self.max_selected {
                        let to_deselect = self
                            .selected_boxes
                            .pop_front()
                            .expect("`selected_boxes` should contain an item.");
                        self.boxes[to_deselect]
                            .widget()
                            .remove_css_class("suggested-action");
                    }
                }

                let new_selection_valid = self.selected_boxes.len() == self.max_selected;

                if self.selection_valid != new_selection_valid {
                    self.selection_valid = new_selection_valid;
                    sender
                        .output(DraftContainerOutput::SelectionValid(self.selection_valid))
                        .expect("Failed to send message `DraftContainerOutput::SelectionValid`.");
                }
            }
            DraftContainerInput::RequestSelected => {
                sender
                    .output(DraftContainerOutput::SelectionComplete(
                        self.selected_boxes
                            .iter()
                            .map(|x| self.boxes[*x].model().cards.clone())
                            .flatten()
                            .collect(),
                    ))
                    .expect("Failed to send message `DraftContainerInput::RequestSelected`");
            }
        }
    }
}
