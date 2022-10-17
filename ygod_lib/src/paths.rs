use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub struct Paths {
    project_dirs: ProjectDirs,
    ext_folder: &'static str,
}

impl Paths {
    pub fn data_dir(&self) -> &Path {
        self.project_dirs.data_dir()
    }

    pub fn ext_dir<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.data_dir().join(self.ext_folder).join(path)
    }
}

pub static PATHS: Lazy<Paths> = Lazy::new(|| {
    let p = Paths {
        project_dirs: ProjectDirs::from("com", "myujiku", "ygod").unwrap(),
        ext_folder: "ext",
    };

    // Ensure "$DATA_DIR/ext"
    fs::create_dir_all(p.data_dir().join(p.ext_folder)).unwrap();

    p
});
