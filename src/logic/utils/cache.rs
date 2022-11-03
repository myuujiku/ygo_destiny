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

use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::logic::ext_data::{banlists, cardinfo, cardsets};

pub struct Cache {
    pub banlists: banlists::BanlistsMetaType,
    pub cardinfo: cardinfo::CardinfoMetaType,
    pub cardsets: cardsets::CardsetsMetaType,
}

impl Cache {
    pub fn get_cards_from_sets(&self, sets: Vec<String>) -> Vec<u32> {
        let mut cards = Vec::new();

        for set in sets {
            cards.extend(self.cardsets[&set].cards.as_ref().unwrap());
        }

        return cards;
    }
}

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    let cache = Cache {
        banlists: banlists::BanlistsMetaType::new(),
        cardinfo: cardinfo::CardinfoMetaType::new(),
        cardsets: cardsets::CardsetsMetaType::new(),
    };
    Mutex::new(cache)
});
