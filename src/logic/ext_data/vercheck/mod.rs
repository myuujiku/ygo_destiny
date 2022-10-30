use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::logic::utils::http::ResponseType;
use crate::logic::utils::paths::PATHS;

pub const EXT_URL: &str = "https://db.ygoprodeck.com/api/v7/checkDBVer.php";

pub static EXT_PATH: Lazy<PathBuf> = Lazy::new(|| PATHS.ext_dir("version.json"));

#[derive(Serialize, Deserialize)]
struct DBVersion {
    last_update: String,
}

pub fn new_update_version_available(version_response: ResponseType) -> Option<String> {
    if version_response.is_ok() {
        // Get old version from file
        let old_version = match EXT_PATH.is_file() {
            true => Some(serde_json::from_str::<DBVersion>(
                fs::read_to_string(&*EXT_PATH).unwrap().as_str(),
            )),
            false => None,
        };

        // Get new version from version_response
        let new_version =
            &serde_json::from_str::<Vec<DBVersion>>(version_response.unwrap().as_str()).unwrap()[0];

        // Compare versions
        if old_version.is_none()
            || old_version.unwrap().unwrap().last_update != new_version.last_update
        {
            return Some(
                serde_json::to_string(&new_version)
                    .unwrap()
                    .as_str()
                    .to_string(),
            );
        } else {
            return None;
        }
    } else {
        return None;
    }
}
