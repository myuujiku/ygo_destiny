use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static BASE_URL: &str = "https://storage.googleapis.com/ygoprodeck.com/";

#[derive(Hash, Eq, PartialEq)]
pub enum ImageType {
    Big,
    Small,
    Cropped,
}

static IMAGE_TYPES: Lazy<Mutex<HashMap<ImageType, &str>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(ImageType::Big, "pics/");
    m.insert(ImageType::Small, "pics_small/");
    m.insert(ImageType::Cropped, "pics_artgame/");
    Mutex::new(m)
});

pub fn get_image(id: String, image_type: ImageType) -> Result<(), Box<dyn std::error::Error>> {
    // Compose url
    let url: String =
        BASE_URL.to_string() + IMAGE_TYPES.lock().unwrap().get(&image_type).unwrap() + &id + ".jpg";

    // Make http request
    let response = reqwest::blocking::get(&url)?;

    println!("{}", response.status());

    Ok(())
}
