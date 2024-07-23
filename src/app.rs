use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::config::{APP_ID, BUILD_TYPE};
use std::{cell::RefCell, fs, rc::Rc};

mod actions;
mod content;
mod modals;
mod pet;
mod settings;

use pet::pet_row;
use pet_row::Sort;

const DATA_FILE_NAME: &str = "data.yaml";

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
    pet_rows: FactoryVecDeque<pet_row::Model>,
    current_view: content::Panes,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    SavePets,
    LoadPets,
    AddPetRow(Rc<RefCell<pet::Pet>>),
    SelectPetRow(usize),
    ShowAddPetPane,

    ShowEditPetView,
    ShowPetDetailsView,
    ApplyChanges,
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
                                                sender.input(Self::Input::SelectPetRow(row.index() as usize));
                                            }
                                        }
                                    },
                                }
                            },
                    }
                },

                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    #[watch] set_title: model.current_view.as_ref(),

                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            pack_start = &gtk::Button {
                                set_label: "Cancel",
                                #[watch] set_visible: model.is_in_edit_mode(),

                                connect_clicked[sender] => move |_| {
                                    sender.input(AppInput::ShowPetDetailsView);
                                },
                            },

                            pack_end = &gtk::Button {
                                set_label: "Edit",
                                #[watch] set_visible: !model.is_in_edit_mode(),

                                connect_clicked[sender] => move |_| {
                                    sender.input(AppInput::ShowEditPetView);
                                },
                            },

                            pack_end = &gtk::Button {
                                set_label: "Apply",
                                add_css_class: "suggested-action",
                                #[watch] set_visible: model.is_in_edit_mode(),

                                connect_clicked[sender] => move |_| {
                                    sender.input(AppInput::ApplyChanges);
                                },
                            },
                        },

                        #[wrap(Some)]
                        set_content = model.content.widget(),
                    }
                }
            },

            connect_show[sender] => move |_| {
                sender.input(Self::Input::LoadPets);
            },

            connect_close_request[sender] => move |_| {
                sender.input(Self::Input::SavePets);
                gtk::Inhibit(false)
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
                    content::ContentOutput::AddPet(pet) => Self::Input::AddPetRow(pet),
                }
            });
        let model = AppModel {
            content,
            pet_rows,
            current_view: content::Panes::PetDetails,
        };

        let pet_list_box = model.pet_rows.widget();
        let widgets = view_output!();

        Self::load_window_state(&widgets);
        Self::create_actions(&widgets, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::SavePets => {
                let pets = self.pet_rows
                    .iter()
                    .map(|pet_row| pet::Pet { name: pet_row.pet.borrow().name.clone() })
                    .collect::<Vec<pet::Pet>>();

                let mut path = gtk::glib::user_data_dir();
                path.push(APP_ID);
                fs::create_dir_all(&path)
                    .expect("Should be able to create directory.");

                path.push(DATA_FILE_NAME);
                let file = fs::File::create(path)
                    .expect("Should be able to create YAML file.");

                serde_yml::to_writer(file, &pets)
                    .expect("Should be able to write data to YAML file");
            }
            Self::Input::LoadPets => {
                let mut path = gtk::glib::user_data_dir();
                path.push(APP_ID);
                path.push(DATA_FILE_NAME);

                if let Ok(file) = fs::File::open(path) {
                    let pets: Vec<pet::Pet> = serde_yml::from_reader(file)
                        .expect("Should be able to read data from YAML file.");

                    for pet in pets {
                        sender.input(Self::Input::AddPetRow(Rc::new(RefCell::new(pet))));
                    }
                } else {
                    // Assume it's the first time using the app.
                }
            }
            Self::Input::AddPetRow(pet) => {
                self.pet_rows.guard().push_sorted(pet_row::Init { pet });
            }
            Self::Input::SelectPetRow(index) => {
                let selected_pet = &self.pet_rows[index].pet;
                self.content
                    .sender()
                    .send(content::ContentInput::ShowPetDetails(Rc::clone(
                        selected_pet
                    )))
                    .expect("Should be able to forward message to child");
            }
            Self::Input::ShowAddPetPane => {
                self.content
                    .sender()
                    .send(content::ContentInput::ShowAddPetPane)
                    .expect("Should be able to forward message to child");
            }

            Self::Input::ShowEditPetView => {
                self.content.sender().send(content::ContentInput::SetVisiblePane(content::Panes::EditPet))
                    .expect("Should be able to send message to child component");
                self.current_view = content::Panes::EditPet;
            }
            Self::Input::ShowPetDetailsView => {
                self.content.sender().send(content::ContentInput::SetVisiblePane(content::Panes::PetDetails))
                    .expect("Should be able to send message to child component");
                self.current_view = content::Panes::PetDetails;
            }
            Self::Input::ApplyChanges => {
                self.content.sender().send(content::ContentInput::ApplyChanges)
                    .expect("Should be able to send message to child component");
                self.current_view = content::Panes::PetDetails;
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(widgets);
    }
}

impl AppModel {
    fn is_in_edit_mode(&self) -> bool {
        match self.current_view {
            content::Panes::EditPet => true,
            content::Panes::PetDetails => false,
        }
    }
}
