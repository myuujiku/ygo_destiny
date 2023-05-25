use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    fs::create_dir_all(&*ygo_destiny::DATA_DIR)?;
    let db = rusqlite::Connection::open(&*ygo_destiny::DB_PATH).unwrap();

    Ok(())
}
