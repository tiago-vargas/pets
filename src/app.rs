use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::config::BUILD_TYPE;

mod actions;
mod content;
mod modals;
mod pet;
mod settings;

use pet::pet_row;

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
    pet_rows: FactoryVecDeque<pet_row::Model>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    AddPet(pet::Pet),

    ShowPreferencesWindow,
    ShowKeyboardShortcutsWindow,
    ShowHelpWindow,
    ShowAboutWindow,
}

#[derive(Debug)]
pub(crate) enum AppOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = AppOutput;

    menu! {
        primary_menu: {
            section! {
                "Preferences" => actions::ShowPreferences,
                "Keyboard Shortcuts" => actions::ShowKeyboardShortcuts,
                "Help" => actions::ShowHelp,
                "About App" => actions::ShowAbout,
            },
        }
    }

    view! {
        main_window = adw::ApplicationWindow {
            set_title: Some("Pets"),

            add_css_class?: if BUILD_TYPE == "debug" { Some("devel") } else { None },

            adw::NavigationSplitView {
                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "Pets",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            pack_end = &gtk::MenuButton {
                                set_icon_name: "open-menu-symbolic",
                                set_menu_model: Some(&primary_menu),
                            },
                        },

                        #[wrap(Some)]
                        set_content = &gtk::ScrolledWindow {
                            #[local_ref]
                            pet_list_box -> gtk::ListBox {
                                add_css_class: "navigation-sidebar",
                            },
                        },
                    }
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    set_title: "Pet Details",

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar { },

                        #[wrap(Some)]
                        set_content = model.content.widget(),
                    }
                }
            },
        }
    }

    fn init(
        _init: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let content = content::ContentModel::builder()
            .launch(content::ContentInit)
            .detach();
        let pet_rows = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());
        let model = AppModel { content, pet_rows };

        let pet_list_box = model.pet_rows.widget();
        let widgets = view_output!();

        Self::load_window_state(&widgets);
        Self::create_actions(&widgets, &sender);

        /* FOR DEBUGGING ONLY */
        sender.input(Self::Input::AddPet(pet::Pet { name: String::from("Pet 1") }));
        sender.input(Self::Input::AddPet(pet::Pet { name: String::from("Pet 2") }));
        sender.input(Self::Input::AddPet(pet::Pet { name: String::from("Pet 3") }));
        sender.input(Self::Input::AddPet(pet::Pet { name: String::from("Pet 4") }));
        sender.input(Self::Input::AddPet(pet::Pet { name: String::from("Pet 5") }));
        /* --- --------- ---- */

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        use modals::{about, help, keyboard_shortcuts, preferences};

        match message {
            Self::Input::AddPet(pet) => {
                self.pet_rows.guard().push_back(pet_row::Init { pet });
            }

            Self::Input::ShowPreferencesWindow => {
                let app = relm4::main_application();
                let main_window = app.windows().first()
                    .expect("Event should have been triggered by last focused window, thus first item")
                    .clone();

                let preferences_window = preferences::Model::builder()
                    .transient_for(&main_window)
                    .launch(preferences::Init)
                    .detach();

                preferences_window.widget().present();
            }
            Self::Input::ShowKeyboardShortcutsWindow => {
                let keyboard_shortcuts_window = keyboard_shortcuts::Model::builder()
                    .launch(keyboard_shortcuts::Init)
                    .detach();
                keyboard_shortcuts_window.widget().present();
            }
            Self::Input::ShowHelpWindow => {
                let help_window = help::Model::builder()
                    .launch(help::Init)
                    .detach();
                help_window.widget().present();
            }
            Self::Input::ShowAboutWindow => {
                let app = relm4::main_application();
                let main_window = app.windows().first()
                    .expect("Event should have been triggered by last focused window, thus first item")
                    .clone();

                let about_window = about::Model::builder()
                    .transient_for(&main_window)
                    .launch(about::Init)
                    .detach();
                about_window.widget().present();
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}
