use crate::{app, config::APP_ID};

use gtk::prelude::*;
use relm4::prelude::*;

pub(crate) enum WindowSettings {
    Width,
    Height,
    Maximized,
}

impl WindowSettings {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Width => "window-width",
            Self::Height => "window-height",
            Self::Maximized => "window-maximized",
        }
    }
}

impl app::AppModel {
    pub(super) fn save_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(APP_ID);

        let (width, height) = widgets.main_window.default_size();
        let _ = settings.set_int(WindowSettings::Width.as_str(), width);
        let _ = settings.set_int(WindowSettings::Height.as_str(), height);

        let _ = settings.set_boolean(
            WindowSettings::Maximized.as_str(),
            widgets.main_window.is_maximized(),
        );
    }

    pub(super) fn load_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(APP_ID);

        let width = settings.int(WindowSettings::Width.as_str());
        let height = settings.int(WindowSettings::Height.as_str());
        widgets.main_window.set_default_size(width, height);

        let maximized = settings.boolean(WindowSettings::Maximized.as_str());
        widgets.main_window.set_maximized(maximized);
    }
}
