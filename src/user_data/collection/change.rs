use serde::{Serialize, Deserialize};

use super::Card;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChangeContent {
    pub cards: Vec<Card>,
    pub date: String,
    pub round: Option<u16>,
}

impl ChangeContent {
    pub fn new(cards: Vec<Card>, date: String, round: Option<u16>) -> Self {
        Self { cards, date, round }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum Change {
    #[default]
    None,
    Add(ChangeContent),
    Remove(ChangeContent),
}
