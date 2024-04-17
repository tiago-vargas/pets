use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::config::BUILD_TYPE;
use std::{cell::RefCell, rc::Rc};

mod actions;
mod content;
mod modals;
mod pet;
mod settings;

use pet::pet_row;
use pet_row::Sort;

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
    pet_rows: FactoryVecDeque<pet_row::Model>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    AddPet(Rc<RefCell<pet::Pet>>),
    SelectPet(usize),
    ShowAddPetPane,

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
                            pack_start = &gtk::Button {
                                set_icon_name: "list-add-symbolic",
                                set_tooltip: "Add a Pet",

                                connect_clicked[sender] => move |_| {
                                    sender.input(Self::Input::ShowAddPetPane);
                                }
                            },

                            pack_end = &gtk::MenuButton {
                                set_icon_name: "open-menu-symbolic",
                                set_menu_model: Some(&primary_menu),
                            },
                        },

                        #[wrap(Some)]
                        set_content =
                            if model.pet_rows.is_empty() {
                                &adw::StatusPage {
                                    set_title: "No Pets Yet",
                                    set_description: Some("Use the + button to add pets."),
                                }
                            } else {
                                &gtk::ScrolledWindow {
                                    #[local_ref]
                                    pet_list_box -> gtk::ListBox {
                                        add_css_class: "navigation-sidebar",

                                        connect_row_selected[sender] => move |_self, row| {
                                            if let Some(row) = row {
                                                sender.input(Self::Input::SelectPet(row.index() as usize));
                                            }
                                        }
                                    },
                                }
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
        let pet_rows =
            FactoryVecDeque::<pet_row::Model>::new(gtk::ListBox::default(), sender.input_sender());
        let content = content::ContentModel::builder()
            .launch(content::ContentInit { pet: None })
            .forward(sender.input_sender(), |response| {
                match response {
                    content::ContentOutput::AddPet(pet) => Self::Input::AddPet(pet),
                }
            });
        let model = AppModel { content, pet_rows };

        let pet_list_box = model.pet_rows.widget();
        let widgets = view_output!();

        Self::load_window_state(&widgets);
        Self::create_actions(&widgets, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        use modals::{about, help, keyboard_shortcuts, preferences};

        match message {
            Self::Input::AddPet(pet) => {
                self.pet_rows.guard().push_sorted(pet_row::Init { pet });
            }
            Self::Input::SelectPet(index) => {
                self.content
                    .sender()
                    .send(content::ContentInput::ShowPet(Rc::clone(
                        &self.pet_rows[index].pet,
                    )))
                    .expect("Should be able to forward message to child");
            }
            Self::Input::ShowAddPetPane => {
                self.content
                    .sender()
                    .send(content::ContentInput::ShowAddPetPane)
                    .expect("Should be able to forward message to child");
            }

            Self::Input::ShowPreferencesWindow => {
                let app = relm4::main_application();
                let main_window = app
                    .windows()
                    .first()
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
                let main_window = app
                    .windows()
                    .first()
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
        Self::save_window_state(widgets);
    }
}
