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
use std::path::PathBuf;

use gtk::glib::Sender;

use crate::logic::utils::CACHE;
use crate::logic::utils::PATHS;

/// Root of the image API endpoint.
static BASE_URL: &str = "https://images.ygoprodeck.com/images/";

/// Enum used to differentiate which kind of card image should be downloaded.
pub enum ImageType {
    /// High resolution card image.
    Big,
    /// Low resolution card image.
    Small,
    /// Cropped card image (card art only).
    Cropped,
}

/// Returns a URL suffix based on `image_type`.
fn get_type_suffix(image_type: &ImageType) -> String {
    match image_type {
        ImageType::Big => "cards/",
        ImageType::Small => "cards_small/",
        ImageType::Cropped => "cards_cropped/",
    }
    .to_string()
}

/// Returns the folder where images of type `image_type` are stored.
fn get_type_path(image_type: &ImageType) -> &PathBuf {
    match image_type {
        ImageType::Big => &PATHS.image_paths.cards_big,
        ImageType::Small => &PATHS.image_paths.cards_small,
        ImageType::Cropped => &PATHS.image_paths.cards_cropped,
    }
}

/// Returns the full URL of the API endpoint based on `image_type`.
pub fn get_url(image_type: ImageType) -> String {
    format!("{}{}", BASE_URL, get_type_suffix(&image_type))
}

/// Downloads all cards that are not present in the file system.
///
/// # Arguments
///
/// * `image_type` – Type of the images to download.
///
/// * `sender` – Gtk object to send the completion status to.
pub fn download_missing_cards(image_type: ImageType, sender: Sender<(f64, String)>) {
    let paths = fs::read_dir(get_type_path(&image_type)).unwrap();

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
                get_type_path(&image_type).join(&filename),
                response.unwrap().bytes().unwrap(),
            )
            .unwrap();
        }

        sender
            .send((
                (i + 1) as f64 / cards_to_download as f64,
                format!("Cards downloaded: {}/{}", i + 1, cards_to_download),
            ))
            .expect("Could not send through channel");
    }
}
