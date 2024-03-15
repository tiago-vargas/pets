use relm4::prelude::*;

mod app;
mod config;

fn main() {
    let app = RelmApp::new(app::APP_ID);
    app.run::<app::AppModel>(());
}
