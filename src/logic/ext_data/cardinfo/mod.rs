use std::collections::HashMap;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::logic::utils::http::CardSetMapType;
use crate::logic::utils::paths::PATHS;

pub type CardinfoMetaType = HashMap<u32, Card>;

pub const EXT_URL: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";

pub static EXT_PATH: Lazy<PathBuf> = Lazy::new(|| PATHS.ext_dir("cardinfo.bin"));

// Representation of one card set of a card from the YGOPRODeck API
#[derive(Serialize, Deserialize, Debug)]
pub struct YGOPDCardSet {
    pub set_name: String,
    pub set_code: String,
    pub set_rarity: String,
}

// Representation of one card from the YGOPRODeck API
#[derive(Serialize, Deserialize, Debug)]
pub struct YGOPDCard {
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
    pub card_sets: Option<Vec<YGOPDCardSet>>,
}

// Representation of the data from the YGOPRODeck API
#[derive(Serialize, Deserialize, Debug)]
pub struct YGOPDData {
    pub data: Vec<YGOPDCard>,
}

// Representation of the data we need
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
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

pub fn parse(cardinfo: &str, card_set_map: &mut CardSetMapType) -> CardinfoMetaType {
    let mut cardinfo_map: CardinfoMetaType = HashMap::new();

    // Iterate of cards in data
    for card in serde_json::from_str::<YGOPDData>(cardinfo).unwrap().data {
        cardinfo_map.insert(
            card.id,
            Card {
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

        // Extract card_sets and put them into card_set_map
        if card.card_sets.is_some() {
            for card_set in card.card_sets.unwrap() {
                let val = card_set_map.get_mut(card_set.set_name.as_str());

                // Check if there already is a Vec at val
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
