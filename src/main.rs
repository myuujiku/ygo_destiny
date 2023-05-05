use ygo_destiny;

fn main() {
    env_logger::init();
    let db = rusqlite::Connection::open(&*ygo_destiny::DB_PATH).unwrap();
    ygo_destiny::db::update_and_log(&db);
}
