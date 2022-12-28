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

use regex::Regex;

/// Banlist element (card with limit number) type.
pub type BanlistElementType = HashMap<u32, u8>;
/// Type contained in a processed banlist binary file.
pub type BanlistsMetaType = HashMap<String, BanlistElementType>;

/// External [URL](https://ygo.anihelp.co.uk/public/config3/TCGCombiList.conf) to the banlist data.
pub const EXT_URL: &str = "https://ygo.anihelp.co.uk/public/config3/TCGCombiList.conf";

/// Returns a processed banlist.
///
/// # Arguments
///
/// * `banlist` – Slice containing raw banlist data.
pub fn parse(banlists: &str) -> BanlistsMetaType {
    // Define regex for seperating the banlists in the TCGCombiList (all official TCG banlists)
    let re_banlist_section = Regex::new(r"!.*\n(?:[\d]* [\d].*\n)+").unwrap();

    // Define regex for detecting the banlist date
    let re_date =
        Regex::new(r"(?P<d>\d{2}).(?P<m>\d{2}).(?P<y>\d{4})|(?P<Y>\d{4}).(?P<M>\d{2})").unwrap();

    // Define container for banlists in the format: <date, <card: limit, ...>>
    let mut banlist_map: BanlistsMetaType = HashMap::new();

    // Parse the given banlist file
    for banlist in re_banlist_section.find_iter(banlists) {
        let mut map: BanlistElementType = HashMap::new();

        // Split the banlist by newlines
        let split: Vec<&str> = banlist.as_str().split("\n").collect();

        // Iterate over the banlist's cards, so everything in the split Vec but the first element, which is the date
        for card in &split[1..] {
            // Split the card by whitespaces
            let card_split: Vec<&str> = card.split(" ").collect();

            // Insert the card's banlist data into the banlist_map if at least 2 elements, an id and a limit value, are given
            if card_split.len() >= 2 {
                map.insert(
                    card_split[0].parse::<u32>().unwrap(),
                    card_split[1].parse::<u8>().unwrap(),
                );
            }
        }

        // Get the banlist date
        let date_captures = re_date.captures(split[0]).unwrap();
        let date = match (
            date_captures.name("d").is_some(),
            date_captures.name("Y").is_some(),
        ) {
            (true, false) => vec![
                date_captures["y"].to_string(),
                date_captures["m"].to_string(),
                date_captures["d"].to_string(),
            ]
            .join("/"),
            (false, true) => vec![
                date_captures["Y"].to_string(),
                date_captures["M"].to_string(),
                "01".to_string(),
            ]
            .join("/"),
            _ => "0000/00/00".to_string(),
        };

        // Add collected data to banlist_map
        banlist_map.insert(date, map);
    }

    return banlist_map;
}
