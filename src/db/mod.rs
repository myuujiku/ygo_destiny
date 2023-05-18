use std::{collections::HashMap, error};

use serde::{Deserialize, Serialize};

mod urls {
    pub const API_CARDINFO: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";
    pub const API_CARDSETS: &str = "https://db.ygoprodeck.com/api/v7/cardsets.php";
}

#[derive(Serialize, Deserialize, Debug)]
struct CardinfoSet {
    set_name: String,
    set_code: String,
    set_rarity: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardinfoCard {
    id: u32,
    card_sets: Option<Vec<CardinfoSet>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardinfoRoot {
    data: Vec<CardinfoCard>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardsetsInfo {
    set_code: String,
    num_of_cards: u16,
    tcg_date: Option<String>,
    set_image: Option<String>,
    cards: Option<Vec<(u32, String)>>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardsetsSet {
    set_name: String,
    #[serde(flatten)]
    info: CardsetsInfo,
}

pub fn update(db: &rusqlite::Connection) -> Result<(), Box<dyn error::Error>> {
    let response = reqwest::blocking::get(urls::API_CARDINFO)?;
    let json_string = response.text()?;

    db.execute("DROP TABLE IF EXISTS cards", ())?;
    db.execute(
        "CREATE TABLE cards (
            id INTEGER PRIMARY KEY,
            name TEXT,
            card_type TEXT,
            description TEXT,
            atk INTEGER,
            def INTEGER,
            level INTEGER,
            type TEXT,
            attribute TEXT,
            archetype TEXT,
            pend_scale INTEGER,
            link_rating INTEGER
        )",
        (),
    )?;

    let sql_columns = [
        ("id", "id"),
        ("name", "name"),
        ("type", "card_type"),
        ("desc", "description"),
        ("atk", "atk"),
        ("def", "def"),
        ("level", "level"),
        ("race", "type"),
        ("attribute", "attribute"),
        ("archetype", "archetype"),
        ("scale", "pend_scale"),
        ("linkval", "link_rating"),
    ]
    .iter()
    .fold("".to_string(), |mut prev, new| {
        let prefix = if prev.is_empty() { "" } else { ", " };
        prev.push_str(&format!(
            "{prefix}json_extract(value, '$.{}') as {}",
            new.0, new.1
        ));
        prev
    });

    db.execute(
        &format!(
            "INSERT INTO cards SELECT {} FROM json_each('{}', '$.data')",
            sql_columns,
            json_string.replace("'", "''"),
        ),
        (),
    )?;

    let response = reqwest::blocking::get(urls::API_CARDSETS)?;

    let mut cardsets: HashMap<String, CardsetsInfo> =
        serde_json::from_str::<Vec<CardsetsSet>>(&response.text()?)?
            .into_iter()
            .map(|x| (x.set_name, x.info))
            .collect();
    println!("{:#?}", cardsets);

    for card in serde_json::from_str::<CardinfoRoot>(&json_string)?.data {
        let Some(sets) = card.card_sets else {
            continue;
        };

        for set in sets {
            if let Some(cardset) = cardsets.get_mut(&set.set_name) {
                let cards = cardset
                    .cards
                    .get_or_insert(vec![]);
                cards.push((card.id, set.set_rarity));
            }
        }
    }
    println!("{:#?}", cardsets);

    Ok(())
}

pub fn update_and_log(db: &rusqlite::Connection) {
    match update(db) {
        Ok(_) => (),
        Err(err) => log::error!("\n{}", err),
    }
}
