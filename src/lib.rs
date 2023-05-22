pub mod db;
pub mod types;

pub const APP_ID_PARTS: (&str, &str, &str) = ("com", "myujiku", "ygo_destiny");

pub static APP_ID: types::Lazy<String> =
    types::Lazy::new(|| format!("{}.{}.{}", APP_ID_PARTS.0, APP_ID_PARTS.1, APP_ID_PARTS.2));

pub static DATA_DIR: types::Lazy<std::path::PathBuf> = types::Lazy::new(|| {
    directories::ProjectDirs::from(APP_ID_PARTS.0, APP_ID_PARTS.1, APP_ID_PARTS.2)
        .expect("Failed to initialise project dirs.")
        .data_dir()
        .to_path_buf()
});

pub static DB_PATH: types::Lazy<std::path::PathBuf> = types::Lazy::new(|| DATA_DIR.join("data.db"));

pub static SETS_PATH: types::Lazy<std::path::PathBuf> =
    types::Lazy::new(|| DATA_DIR.join("sets.bin"));

pub static BINCODE_CONFIG: types::BincodeConfig = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();
