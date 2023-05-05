mod api_data;
pub use api_data::*;

use std::error;

mod urls {
    pub const API: &str = "https://db.ygoprodeck.com/api/v7/cardinfo.php";
}

pub fn update(db: &rusqlite::Connection) -> Result<(), Box<dyn error::Error>> {
    let response = reqwest::blocking::get(urls::API)?;

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
            // Replace readfile somehome yikes
            "INSERT INTO cards SELECT {} FROM json_each('{}', '$.data')",
            sql_columns,
            response.text()?.replace("'", "''"),
        ),
        (),
    )?;

    Ok(())
}

pub fn update_and_log(db: &rusqlite::Connection) {
    match update(db) {
        Ok(_) => (),
        Err(err) => log::error!("\n{}", err),
    }
}
