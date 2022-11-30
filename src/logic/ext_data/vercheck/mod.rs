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

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::logic::utils::http::ResponseType;
use crate::logic::utils::paths::PATHS;

/// External [URL](https://db.ygoprodeck.com/api/v7/checkDBVer.php) to the version data.
pub const EXT_URL: &str = "https://db.ygoprodeck.com/api/v7/checkDBVer.php";

/// Path of the json file containing the current version.
pub static EXT_PATH: Lazy<PathBuf> = Lazy::new(|| PATHS.ext_dir("version.json"));

/// Representation of a database version from the YGOPRODECK API.
#[derive(Serialize, Deserialize)]
struct DBVersion {
    last_update: String,
}

/// Returns the date of the new version if an update is available.
///
/// # Arguments
///
/// * `version_response` – Response from the http request to [`EXT_URL`].
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
