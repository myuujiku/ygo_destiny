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

use std::fs;

use crate::logic::utils::cache::CACHE;
use crate::logic::utils::paths::PATHS;

static BASE_URL: &str = "https://storage.googleapis.com/ygoprodeck.com/";

pub enum ImageType {
    Big,
    Small,
    Cropped,
}

fn get_type_suffix(image_type: &ImageType) -> String {
    match image_type {
        ImageType::Big => "pics/",
        ImageType::Small => "pics_small/",
        ImageType::Cropped => "pics_artgame/",
    }
    .to_string()
}

fn get_type_folder(image_type: &ImageType) -> String {
    match image_type {
        ImageType::Big => PATHS.img_big_folder,
        ImageType::Small => PATHS.img_small_folder,
        ImageType::Cropped => PATHS.img_cropped_folder,
    }
    .to_string()
}

fn get_url(image_type: ImageType) -> String {
    format!("{}{}", BASE_URL, get_type_suffix(&image_type))
}

pub fn download_missing_cards(image_type: ImageType) {
    let paths = fs::read_dir(PATHS.img_dir().join(get_type_folder(&image_type))).unwrap();

    let existing_images: Vec<u32> = paths
        .filter_map(|x| {
            let p = x.unwrap().path();
            let ext = p.extension();
            if ext.is_some() && ext.unwrap() == "jpg" {
                p.file_stem().unwrap().to_str().unwrap().parse::<u32>().ok()
            } else {
                None
            }
        })
        .collect();

    let cardinfo = &CACHE.lock().unwrap().cardinfo;
    let missing_cards: Vec<&u32> = cardinfo
        .keys()
        .filter(|x| !existing_images.contains(x))
        .collect();

    let cards_to_download = missing_cards.len();

    let client = reqwest::blocking::Client::new();

    for (i, card_id) in missing_cards.iter().enumerate() {
        let filename = format!("{}.jpg", card_id);
        let url = format!("{}{}{}", BASE_URL, get_type_suffix(&image_type), filename);
        let response = client.get(url).send();

        //println!("{:#?}", response);

        if response.is_ok() {
            fs::write(
                PATHS
                    .img_dir()
                    .join(get_type_folder(&image_type))
                    .join(&filename),
                response.unwrap().bytes().unwrap(),
            )
            .unwrap();
        }

        println!("{}/{} {}%", i+1, cards_to_download, ((i+1) as f64 / cards_to_download as f64 * 100.0) as usize);
    }
}
