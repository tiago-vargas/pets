use relm4::{
    actions::{RelmAction, RelmActionGroup},
    prelude::*,
};

use super::{modals, AppModel};
use modals::Modal;

relm4::new_action_group!(pub(crate) AppActions, "app");

relm4::new_stateless_action!(pub(crate) ShowAbout, AppActions, "about");

impl AppModel {
    pub(crate) fn create_actions(
        widgets: &<Self as SimpleComponent>::Widgets,
        _sender: &ComponentSender<Self>,
    ) {
        let mut app_actions = RelmActionGroup::<AppActions>::new();

        let show_about = RelmAction::<ShowAbout>::new_stateless(move |_| {
            modals::show_window(Modal::About);
        });
        app_actions.add_action(show_about);

        app_actions.register_for_widget(&widgets.main_window);
    }
}
