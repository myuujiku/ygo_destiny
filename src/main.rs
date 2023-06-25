use adw::prelude::ApplicationExt;
use relm4::prelude::*;

use ygo_destiny::{
    data::{app_id, dirs, files},
    ui,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    dirs::init()?;
    let db = rusqlite::Connection::open(files::DB.as_path()).unwrap();

    let main_app = relm4::main_application();
    main_app.set_application_id(Some(app_id::DOT_SEPARATED.as_str()));
    main_app.set_resource_base_path(Some(app_id::SLASH_SEPARATED.as_str()));

    let relm_app = RelmApp::from_app(main_app);
    relm_app.run::<ui::App>(());

    Ok(())
}
