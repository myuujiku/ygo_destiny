use std::fs;

use adw::prelude::ApplicationExt;
use relm4::prelude::*;

use ygo_destiny::ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    fs::create_dir_all(&*ygo_destiny::DATA_DIR)?;
    let db = rusqlite::Connection::open(&*ygo_destiny::DB_PATH).unwrap();

    let main_app = relm4::main_application();
    main_app.set_application_id(Some(&*ygo_destiny::APP_ID));
    main_app.set_resource_base_path(Some(&*ygo_destiny::APP_BASE_PATH));

    let relm_app = RelmApp::from_app(main_app);
    relm_app.run::<ui::App>(());

    Ok(())
}
