use serde::{Deserialize, Serialize};

pub type CardType = u32;

#[derive(Serialize, Deserialize, Clone, Default, Debug, Eq, Hash, PartialEq)]
pub struct Card {
    pub id: CardType,
}
