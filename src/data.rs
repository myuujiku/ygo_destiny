use bincode::config::{BigEndian, Configuration, Fixint};
use once_cell::sync::Lazy;

pub const APP_ID_PARTS: (&str, &str, &str) = ("com", "myujiku", "ygo_destiny");
pub static BINCODE_CONFIG: Configuration<BigEndian, Fixint> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub mod app_id {
    use super::Lazy;

    use super::APP_ID_PARTS as ID;

    pub static DOT_SEPARATED: Lazy<String> = Lazy::new(|| format!("{}.{}.{}", ID.0, ID.1, ID.2));
    pub static SLASH_SEPARATED: Lazy<String> =
        Lazy::new(|| format!("/{}/{}/{}/", ID.0, ID.1, ID.2));
}

pub mod dirs {
    use std::fs;
    use std::path::PathBuf;

    use super::Lazy;
    use directories::ProjectDirs;

    use super::APP_ID_PARTS as ID;

    pub static ROOT: Lazy<PathBuf> = Lazy::new(|| {
        ProjectDirs::from(ID.0, ID.1, ID.2)
            .expect("Failed to initialise project directories.")
            .data_dir()
            .to_path_buf()
    });

    pub static USER: Lazy<PathBuf> = Lazy::new(|| ROOT.join("user"));
    pub static COLLECTIONS: Lazy<PathBuf> = Lazy::new(|| ROOT.join("collections"));
    pub static IMAGES: Lazy<PathBuf> = Lazy::new(|| ROOT.join("images"));

    macro_rules! create_lazy_dirs {
        ( $( $i:ident ),* ) => {
            $(
                fs::create_dir_all($i.as_path())?;
            )*
        }
    }

    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        create_lazy_dirs!(ROOT, USER, COLLECTIONS, IMAGES);

        Ok(())
    }
}

pub mod files {
    use std::path::PathBuf;

    use super::{dirs, Lazy};

    pub static DB: Lazy<PathBuf> = Lazy::new(|| dirs::ROOT.join("data.db"));
}

pub mod images {
    use gtk::Image;
    use relm4::gtk;

    use super::dirs;

    pub fn load_card(id: u32) -> Image {
        let filename = dirs::IMAGES.join(format!("{}.jpg", id));

        Image::from_file(filename)
    }
}
