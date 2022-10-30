/*
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod eval;

use std::collections::HashMap;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::logic::utils::http::CardSetMapType;
use crate::logic::utils::paths::PATHS;

use eval::eval_tags;

pub type CardsetsMetaType = HashMap<String, Set>;

pub const EXT_URL: &str = "https://db.ygoprodeck.com/api/v7/cardsets.php";

pub static EXT_PATH: Lazy<PathBuf> = Lazy::new(|| PATHS.ext_dir("cardsets.bin"));

#[derive(Serialize, Deserialize)]
pub struct YGOPDSet {
    pub set_name: String,
    pub set_code: String,
    pub tcg_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Set {
    pub cards: Option<Vec<u32>>,
    pub code: String,
    pub date: Option<String>,
    pub tags: Vec<String>,
}

pub fn parse(cardsets: &str, card_set_map: CardSetMapType) -> CardsetsMetaType {
    let mut cardsets_map: CardsetsMetaType = HashMap::new();

    for cardset in serde_json::from_str::<Vec<YGOPDSet>>(cardsets).unwrap() {
        let cards = {
            // Get cards at set_name
            let cards_tmp = card_set_map.get(&cardset.set_name);

            if cards_tmp.is_some() {
                Some(cards_tmp.unwrap().clone())
            } else {
                Some(vec![])
            }
        };

        cardsets_map.insert(
            cardset.set_name.clone(),
            Set {
                cards: cards,
                code: cardset.set_code,
                date: cardset.tcg_date,
                tags: eval_tags(cardset.set_name),
            },
        );
    }

    return cardsets_map;
}
