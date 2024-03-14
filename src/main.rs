use relm4::prelude::*;

mod app;
mod config;

use config::APP_ID;

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<app::AppModel>(());
}
