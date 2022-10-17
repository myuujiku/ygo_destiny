use std::fs;

use serde::{Deserialize, Serialize};

use super::get_response;
use crate::paths::PATHS;

const URL: &str = "https://db.ygoprodeck.com/api/v7/checkDBVer.php";

#[derive(Deserialize, Serialize)]
struct DBVersion {
    last_update: String,
}

pub fn new_version_available() -> bool {
    let version = get_response(URL);

    if version.is_ok() {
        let path = PATHS.ext_dir("version.json");
        let old_version = match path.is_file() {
            true => Some(serde_json::from_str::<DBVersion>(
                fs::read_to_string(&path).unwrap().as_str(),
            )),
            false => None,
        };
        let new_version =
            &serde_json::from_str::<Vec<DBVersion>>(version.unwrap().as_str()).unwrap()[0];

        if old_version.is_none()
            || old_version.unwrap().unwrap().last_update != new_version.last_update
        {
            fs::write(&path, serde_json::to_string(&new_version).unwrap().as_str()).unwrap();
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}
