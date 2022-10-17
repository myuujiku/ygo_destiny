mod eval;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::cardinfo::CardSetType;
use super::get_response;
use eval::eval_tags;

type SetType = HashMap<String, SetYGOD>;

pub const URL: &str = "https://db.ygoprodeck.com/api/v7/cardsets.php";

#[derive(Deserialize, Serialize)]
pub struct Set {
    pub set_name: String,
    pub set_code: String,
    pub tcg_date: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SetYGOD {
    pub cards: Option<Vec<u32>>,
    pub code: String,
    pub date: Option<String>,
    pub tags: Vec<String>,
}

pub fn parse(cardsets: &str, card_set_map: CardSetType) -> SetType {
    let mut cardsets_map: SetType = HashMap::new();

    for cardset in serde_json::from_str::<Vec<Set>>(cardsets).unwrap() {
        let cards = {
            let cards_tmp = card_set_map.get(&cardset.set_name);

            if cards_tmp.is_some() {
                Some(cards_tmp.unwrap().clone())
            } else {
                Some(vec![])
            }
        };

        cardsets_map.insert(
            cardset.set_name.clone(),
            SetYGOD {
                cards: cards,
                code: cardset.set_code,
                date: cardset.tcg_date,
                tags: eval_tags(cardset.set_name),
            },
        );
    }

    return cardsets_map;
}
