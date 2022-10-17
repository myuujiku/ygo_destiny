use std::collections::HashMap;

use regex::Regex;

type BanlistType = HashMap<u32, u8>;
type BanlistListType = HashMap<String, BanlistType>;

pub const URL: &str = "https://ygo.anihelp.co.uk/public/config3/TCGCombiList.conf";

pub fn parse(banlists: &str) -> BanlistListType {
    let re = Regex::new(r"!.*\n(?:[\d]* [\d].*\n)+").unwrap();

    let re_date =
        Regex::new(r"(?P<d>\d{2}).(?P<m>\d{2}).(?P<y>\d{4})|(?P<Y>\d{4}).(?P<M>\d{2})").unwrap();

    // Every banlist in the format {date: {card: limit, ...}}
    let mut banlist_map: BanlistListType = HashMap::new();

    // Parse banlist file
    for banlist in re.find_iter(banlists) {
        let mut map: BanlistType = HashMap::new();
        let split: Vec<&str> = banlist.as_str().split("\n").collect();

        for card in &split[1..] {
            let card_split: Vec<&str> = card.split(" ").collect();
            if card_split.len() >= 2 {
                map.insert(
                    card_split[0].parse::<u32>().unwrap(),
                    card_split[1].parse::<u8>().unwrap(),
                );
            }
        }

        // Get date
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

    banlist_map
}
