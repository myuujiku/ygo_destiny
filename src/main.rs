use adw::prelude::ApplicationExt;
use once_cell::sync::Lazy;
use relm4::prelude::*;
use rusqlite::Connection;

use ygo_destiny::{
    data::{app_id, dirs, files, get_or_log},
    db, ui,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dirs::init()?;

    let db_exists = files::DB.is_file();
    let conn = Lazy::new(|| Connection::open(files::DB.as_path()).unwrap());

    if !db_exists {
        match files::DB_BACKUP.is_file() {
            true => get_or_log(db::restore_backup(), ()),
            false => get_or_log(db::update(&conn), ()),
        }
    }

    let main_app = relm4::main_application();
    main_app.set_application_id(Some(app_id::DOT_SEPARATED.as_str()));
    main_app.set_resource_base_path(Some(app_id::SLASH_SEPARATED.as_str()));

    let relm_app = RelmApp::from_app(main_app);
    relm_app.run::<ui::App>(
        match Lazy::into_value(conn) {
            Ok(val) => val,
            Err(func) => func(),
        }
    );

    Ok(())
}
