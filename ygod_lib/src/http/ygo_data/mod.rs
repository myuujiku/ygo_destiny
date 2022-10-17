mod banlists;
mod cardinfo;
mod cardsets;
mod version;

use std::collections::HashMap;
use std::fs;

use regex::Regex;

use crate::paths::PATHS;

type ResponseType = Result<String, reqwest::Error>;

pub enum UpdateStatus {
    Complete,
    Failed,
    Incomplete,
    UpToDate,
}

struct Responses {
    banlists: ResponseType,
    cardinfo: ResponseType,
    cardsets: ResponseType,
}

// Update external data; return true if successful and false otherwise
pub fn update() -> UpdateStatus {
    if !version::new_version_available() {
        return UpdateStatus::UpToDate;
    }

    let data: Responses = get_data();

    // Exit if the result of get_data is an error
    if data.banlists.is_err() || data.cardinfo.is_err() || data.cardsets.is_err() {
        return UpdateStatus::Failed;
    }

    let banlists: String = Regex::new(r"#.*\n")
        .unwrap()
        .replace_all(data.banlists.unwrap().as_str(), "")
        .to_string();
    let cardinfo: String = data.cardinfo.unwrap();
    let cardsets: String = data.cardsets.unwrap();

    // Parse responses
    let parsed_banlists = banlists::parse(banlists.as_str());

    let mut card_set_map: cardinfo::CardSetType = HashMap::new();
    let parsed_cardinfo = cardinfo::parse(cardinfo.as_str(), &mut card_set_map);
    let parsed_cardsets = cardsets::parse(cardsets.as_str(), card_set_map);

    // Write files
    use bincode::serde::encode_to_vec as encode;
    let config = bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()
        .write_fixed_array_length();

    if [
        fs::write(
            PATHS.ext_dir("banlists.bin"),
            encode(&parsed_banlists, config).unwrap(),
        ),
        fs::write(
            PATHS.ext_dir("cardinfo.bin"),
            encode(&parsed_cardinfo, config).unwrap(),
        ),
        fs::write(
            PATHS.ext_dir("cardsets.bin"),
            encode(&parsed_cardsets, config).unwrap(),
        ),
    ]
    .iter()
    .any(|i| i.is_err())
    {
        return UpdateStatus::Incomplete;
    }

    return UpdateStatus::Complete;
}

fn get_data() -> Responses {
    Responses {
        banlists: get_response(banlists::URL),
        cardinfo: get_response(cardinfo::URL),
        cardsets: get_response(cardsets::URL),
    }
}

fn get_response(url: &str) -> ResponseType {
    Ok(reqwest::blocking::get(url)?.text()?)
}
