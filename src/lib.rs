pub mod db;

pub const APP_ID_PARTS: (&str, &str, &str) = ("com", "myujiku", "ygo_destiny");

pub static APP_ID: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    format!("{}.{}.{}", APP_ID_PARTS.0, APP_ID_PARTS.1, APP_ID_PARTS.2)
});
pub static DATA_DIR: once_cell::sync::Lazy<std::path::PathBuf> = once_cell::sync::Lazy::new(|| {
    directories::ProjectDirs::from(APP_ID_PARTS.0, APP_ID_PARTS.1, APP_ID_PARTS.2)
        .expect("Failed to initialise project dirs.")
        .data_dir()
        .to_path_buf()
});
pub static DB_PATH: once_cell::sync::Lazy<std::path::PathBuf> = once_cell::sync::Lazy::new(|| {
    DATA_DIR.join("data.db")
});
pub static CARDINFO_JSON: once_cell::sync::Lazy<std::path::PathBuf> = once_cell::sync::Lazy::new(|| {
    DATA_DIR.join("cardinfo.json")
});
