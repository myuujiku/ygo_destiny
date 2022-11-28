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

use std::collections::HashMap;
use std::fs;

use bincode::config::{BigEndian, Configuration, Fixint};
use bincode::serde::decode_from_slice as decode;
use bincode::serde::encode_to_vec as encode;
use regex::Regex;

use crate::logic::ext_data::{banlists, cardinfo, cardsets, vercheck};
use crate::logic::utils::cache::CACHE;

pub type ResponseType = Result<String, reqwest::Error>;
pub type CardSetMapType = HashMap<String, Vec<u32>>;

static BINCODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding()
    .write_fixed_array_length();

#[derive(Debug)]
pub enum UpdateStatus {
    Complete,
    Failed,
    Incomplete,
}

struct Responses {
    banlists: ResponseType,
    cardinfo: ResponseType,
    cardsets: ResponseType,
}

pub fn update_version() -> Option<String> {
    let version_response: ResponseType = get_response(vercheck::EXT_URL);

    return vercheck::new_update_version_available(version_response);
}

pub fn update() -> UpdateStatus {
    let data: Responses = get_data();

    // Exit if the Result of get_data is an error
    if data.banlists.is_err() || data.cardinfo.is_err() || data.cardsets.is_err() {
        return UpdateStatus::Failed;
    }

    // Remove lines starting with '#' from banlists response text
    let banlists_raw: String = Regex::new(r"#.*\n")
        .unwrap()
        .replace_all(data.banlists.unwrap().as_str(), "")
        .to_string();
    let cardinfo_raw: String = data.cardinfo.unwrap();
    let cardsets_raw: String = data.cardsets.unwrap();

    // Parse responses
    let parsed_banlists = banlists::parse(banlists_raw.as_str());

    let mut card_set_map: CardSetMapType = HashMap::new();
    let parsed_cardinfo = cardinfo::parse(cardinfo_raw.as_str(), &mut card_set_map);
    let parsed_cardsets = cardsets::parse(cardsets_raw.as_str(), card_set_map);

    // Write binary files

    if [
        fs::write(
            &*banlists::EXT_PATH,
            encode(&parsed_banlists, BINCODE_CONFIG).unwrap(),
        ),
        fs::write(
            &*cardinfo::EXT_PATH,
            encode(&parsed_cardinfo, BINCODE_CONFIG).unwrap(),
        ),
        fs::write(
            &*cardsets::EXT_PATH,
            encode(&parsed_cardsets, BINCODE_CONFIG).unwrap(),
        ),
    ]
    .iter()
    .any(|f| f.is_err())
    {
        return UpdateStatus::Incomplete;
    }

    update_cache(parsed_banlists, parsed_cardinfo, parsed_cardsets);

    return UpdateStatus::Complete;
}

pub fn get_response(url: &str) -> ResponseType {
    Ok(reqwest::blocking::get(url)?.text()?)
}

pub fn load_local_data() {
    // Read files to a vec
    let files = vec![
        fs::read(&*banlists::EXT_PATH),
        fs::read(&*cardinfo::EXT_PATH),
        fs::read(&*cardsets::EXT_PATH),
    ];

    // Check if any errors occured
    if files.iter().any(|f| f.is_err()) {
        // Files don't seem to be complete, so do an update
        update();

        // Save the update version so that the data is not redownloaded immediately
        fs::write(&*vercheck::EXT_PATH, update_version().unwrap()).unwrap();

        return;
    }

    // Decode file contents
    let banlists = decode(files[0].as_ref().unwrap().as_ref(), BINCODE_CONFIG)
        .unwrap()
        .0;

    let cardinfo = decode(files[1].as_ref().unwrap().as_ref(), BINCODE_CONFIG)
        .unwrap()
        .0;

    let cardsets = decode(files[2].as_ref().unwrap().as_ref(), BINCODE_CONFIG)
        .unwrap()
        .0;

    update_cache(banlists, cardinfo, cardsets);
}

fn update_cache(
    banlists: banlists::BanlistsMetaType,
    cardinfo: cardinfo::CardinfoMetaType,
    cardsets: cardsets::CardsetsMetaType,
) {
    CACHE.lock().unwrap().banlists = banlists;
    CACHE.lock().unwrap().cardinfo = cardinfo;
    CACHE.lock().unwrap().cardsets = cardsets;
}

fn get_data() -> Responses {
    Responses {
        banlists: get_response(&banlists::EXT_URL),
        cardinfo: get_response(&cardinfo::EXT_URL),
        cardsets: get_response(&cardsets::EXT_URL),
    }
}
