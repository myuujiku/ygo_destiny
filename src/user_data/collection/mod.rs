mod card;
pub use card::*;

mod change;
pub use change::*;

mod meta_data;
pub use meta_data::*;

use std::collections::HashMap;
use std::fs;

use bincode::{
    config::{BigEndian, Configuration, Fixint},
    serde::decode_from_slice,
    serde::encode_to_vec,
};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use crate::{COLLECTIONS_DIR, BINCODE_CONFIG};

pub static LAST_CHANGED_FORMAT: &str = "%Y%m%d_%H%M%S";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Collection {
    pub meta_data: MetaData,
    pub cards: HashMap<Card, u8>,
    pub changes: Vec<Change>,
    pub tags: HashMap<String, Vec<CardType>>
}

impl Collection {
    pub fn get_names() -> Vec<String> {
        if let Ok(read_dir) = COLLECTIONS_DIR.read_dir() {
            read_dir.map(|path| {
                path.expect(&format!("Failed to read path."))
                    .file_name()
                    .into_string()
                    .expect("Failed to get file name.")
            })
            .collect()
        } else {
            Vec::new()
        }
    }

    pub fn from_name(name: &String) -> Self {
        decode_from_slice(
            &fs::read(COLLECTIONS_DIR.join(name))
                .expect("Failed to read collection."),
            BINCODE_CONFIG,
        )
        .expect("Failed to decode collection.")
        .0
    }

    pub fn save(&mut self, name: &String) {
        self.meta_data.last_changed = format!("{}", Utc::now().format(LAST_CHANGED_FORMAT));
        fs::write(
            COLLECTIONS_DIR.join(name),
            encode_to_vec(self, BINCODE_CONFIG).unwrap(),
        )
        .expect("Failed to save collection.");
    }

    pub fn get_metadata_from(name: &String) -> MetaData {
        decode_from_slice(
            &fs::read(COLLECTIONS_DIR.join(name))
                .expect("Failed to read collection."), 
            BINCODE_CONFIG,
        )
        .expect("Failed to decode collection.")
        .0
    }

    pub fn add_change(&mut self, change: Change) {
        match &change {
            Change::Add(content) => self.add_cards(&content.cards),
            Change::Remove(content) =>  self.remove_cards(&content.cards),
            _ => return,
        }

        self.changes.push(change);
    }

    pub fn undo_change(&mut self) {
        let change = self.changes.pop().unwrap();

        match change {
            Change::Add(content) => self.remove_cards(&content.cards),
            Change::Remove(content) => self.add_cards(&content.cards),
            _ => (),
        }
    }

    fn add_cards(&mut self, cards: &Vec<Card>) {
        for card in cards.iter() {
            if let Some(quantity) = self.cards.get(card) {
                self.cards.insert(card.clone(), quantity + 1);
            } else {
                self.cards.insert(card.clone(), 1);
            }
        }
    }

    fn remove_cards(&mut self, cards: &Vec<Card>) {
        for card in cards.iter() {
            if let Some(quantity) = self.cards.get(card) {
                if quantity < &1 {
                    self.cards.remove(card);
                } else {
                    self.cards.insert(card.clone(), quantity - 1);
                }
            }
        }
    }
}
