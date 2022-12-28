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
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub struct ExtData {
    pub banlists: PathBuf,
    pub cardinfo: PathBuf,
    pub cardsets: PathBuf,
    pub version: PathBuf,
}

impl ExtData {
    pub fn new(root: PathBuf) -> Self {
        Self {
            banlists: root.join("banlists.bin"),
            cardinfo: root.join("cardinfo.bin"),
            cardsets: root.join("cardsets.bin"),
            version: root.join("version.json"),
        }
    }

    pub fn ensure(&self) {
        fs::create_dir_all(self.banlists.parent().unwrap()).unwrap();
    }
}

pub struct ImagePaths {
    pub cards_big: PathBuf,
    pub cards_small: PathBuf,
    pub cards_cropped: PathBuf,
    pub products: PathBuf,
}

impl ImagePaths {
    pub fn new(root: PathBuf) -> Self {
        Self {
            cards_big: root.join("big"),
            cards_small: root.join("small"),
            cards_cropped: root.join("cropped"),
            products: root.join("products"),
        }
    }

    pub fn ensure(&self) {
        fs::create_dir_all(&self.cards_big).unwrap();
        fs::create_dir_all(&self.cards_small).unwrap();
        fs::create_dir_all(&self.cards_cropped).unwrap();
        fs::create_dir_all(&self.products).unwrap();
    }
}

pub struct UserPaths {
    pub collections: PathBuf,
}

impl UserPaths {
    pub fn new(root: PathBuf) -> Self {
        Self {
            collections: root.join("collections"),
        }
    }

    pub fn ensure(&self) {
        fs::create_dir_all(&self.collections).unwrap();
    }
}

/// Container for file system paths used by YGO Destiny. This should generally only be accessed via
/// [`PATHS`].
pub struct Paths {
    pub ext_data: ExtData,
    pub image_paths: ImagePaths,
    pub user_paths: UserPaths,
}

impl Paths {
    pub fn new() -> Self {
        let project_dirs = ProjectDirs::from("com", "myujiku", "ygo_destiny").unwrap();
        let data_dir = project_dirs.data_dir();

        Self {
            ext_data: ExtData::new(data_dir.join("external")),
            image_paths: ImagePaths::new(data_dir.join("images")),
            user_paths: UserPaths::new(data_dir.join("user")),
        }.ensured()
    }

    pub fn ensured(self) -> Self {
        self.ext_data.ensure();
        self.image_paths.ensure();
        self.user_paths.ensure();
        self
    }

    pub fn get_img_dir(&self) -> &Path {
        self.image_paths.cards_big.as_path()
    }
}

/// Paths data container. See [`Paths`] for methods and fields.
pub static PATHS: Lazy<Paths> = Lazy::new(|| {
    Paths::new()
});
