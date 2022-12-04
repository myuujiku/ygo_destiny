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

//! # Examples
//!
//! ```rust
//! use ygo_destiny::logic::utils::cache::CACHE;
//!
//! let banlists = &CACHE.lock().unwrap().banlists;
//! let cardinfo = &CACHE.lock().unwrap().cardinfo;
//! let cardsets = &CACHE.lock().unwrap().cardsets;
//!
//! // Get all cards from the 3 Battle Pack main sets
//! let battle_pack_cards = CACHE.lock().unwrap().get_cards_from_sets(
//!     vec![
//!         "Battle Pack: Epic Dawn".to_string(),
//!         "Battle Pack 2: War of the Giants".to_string(),
//!         "Battle Pack 3: Monster League".to_string(),
//!     ]
//! );

use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::logic::ext_data::{banlists, cardinfo, cardsets};

/// Container for global variables. This should generally only be accessed via [`CACHE`].
pub struct Cache {
    pub banlists: banlists::BanlistsMetaType,
    pub cardinfo: cardinfo::CardinfoMetaType,
    pub cardsets: cardsets::CardsetsMetaType,
}

impl Cache {
    /// Turns a listing of card set names into a listing of all card ids in those sets by looking
    /// them up in `self.cardsets`. Duplicates are *not* handled and must be dealt with externally.
    ///
    /// # Arguments
    ///
    /// * `sets` – A listing of card set names.
    pub fn get_cards_from_sets(&self, sets: Vec<String>) -> Vec<u32> {
        let mut cards = Vec::new();

        for set in sets {
            cards.extend(self.cardsets[&set].cards.as_ref().unwrap());
        }

        return cards;
    }
}

/// Cache data container. See [`Cache`] for methods and fields.
pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    let cache = Cache {
        banlists: banlists::BanlistsMetaType::new(),
        cardinfo: cardinfo::CardinfoMetaType::new(),
        cardsets: cardsets::CardsetsMetaType::new(),
    };
    Mutex::new(cache)
});
