use std::collections::HashMap;

use serde::{Deserialize, Serialize};

type CardInfoType = HashMap<u32, CardYGOD>;
pub type CardSetType = HashMap<String, Vec<u32>>;

pub const URL: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";

#[derive(Deserialize, Serialize)]
pub struct CardSet {
    pub set_name: String,
    pub set_code: String,
    pub set_rarity: String,
}

// Representation of the individual cards' data we need from the YGOPRODeck API
#[derive(Deserialize, Serialize)]
pub struct Card {
    pub id: u32,
    pub name: String,
    pub r#type: String,
    pub desc: String,
    pub atk: Option<u32>,
    pub def: Option<u32>,
    pub level: Option<u8>,
    pub race: String,
    pub attribute: Option<String>,
    pub archetype: Option<String>,
    pub scale: Option<u8>,
    pub linkval: Option<u8>,
    pub card_sets: Option<Vec<CardSet>>,
}

// Representation of the data from the YGOPRODeck API
#[derive(Deserialize, Serialize)]
pub struct Data {
    pub data: Vec<Card>,
}

#[derive(Deserialize, Serialize)]
pub struct CardYGOD {
    pub id: u32,
    pub name: String,
    pub card_type: String,
    pub description: String,
    pub atk: Option<u32>,
    pub def: Option<u32>,
    pub level: Option<u8>,
    pub r#type: String,
    pub attribute: Option<String>,
    pub archetype: Option<String>,
    pub pend_scale: Option<u8>,
    pub link_rating: Option<u8>,
}

pub fn parse(cardinfo: &str, card_set_map: &mut CardSetType) -> CardInfoType {
    let mut cardinfo_map: CardInfoType = HashMap::new();

    for card in serde_json::from_str::<Data>(cardinfo).unwrap().data {
        cardinfo_map.insert(
            card.id,
            CardYGOD {
                id: card.id,
                name: card.name,
                card_type: card.r#type,
                description: card.desc,
                atk: card.atk,
                def: card.def,
                level: card.level,
                attribute: card.attribute,
                r#type: card.race,
                archetype: card.archetype,
                pend_scale: card.scale,
                link_rating: card.linkval,
            },
        );

        // Insert card's card sets into card_set_map
        if card.card_sets.is_some() {
            for card_set in card.card_sets.unwrap() {
                let val = card_set_map.get_mut(card_set.set_name.as_str());

                if val.is_some() {
                    val.unwrap().push(card.id);
                } else {
                    card_set_map.insert(card_set.set_name, vec![card.id]);
                }
            }
        }
    }

    return cardinfo_map;
}
