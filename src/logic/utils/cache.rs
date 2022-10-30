use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::logic::ext_data::{banlists, cardinfo, cardsets};

pub struct Cache {
    pub banlists: banlists::BanlistsMetaType,
    pub cardinfo: cardinfo::CardinfoMetaType,
    pub cardsets: cardsets::CardsetsMetaType,
}

pub static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    let cache = Cache {
        banlists: banlists::BanlistsMetaType::new(),
        cardinfo: cardinfo::CardinfoMetaType::new(),
        cardsets: cardsets::CardsetsMetaType::new(),
    };
    Mutex::new(cache)
});
