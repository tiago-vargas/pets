use adw::prelude::*;
use gtk::glib;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::config::{APP_ID, BUILD_TYPE};
use std::{cell::RefCell, fs, rc::Rc};

mod actions;
mod content;
mod modals;
mod pet;
mod settings;

use content::Panes;
use pet::{pet_row, Pet};
use pet_row::Sort;

const DATA_FILE_NAME: &str = "data.yaml";

pub(crate) struct Model {
	content: Controller<content::Model>,
	pet_rows: FactoryVecDeque<pet_row::Model>,
	selected_row: Option<usize>,
}

#[derive(Debug)]
pub(crate) enum Input {
	SavePets,
	LoadPets,
	AddPetRow(Rc<RefCell<Pet>>),
	SelectPetRow(usize),
	ShowAddPetPane,
	RemoveSelectedPetRow,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = ();
	type Input = Input;
	type Output = ();

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
				#[watch] set_collapsed: model.pet_rows.is_empty(),
				#[watch] set_show_content: matches!(model.content.model().visible_pane, Panes::AddPet),

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
								},
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
										},
									},
								}
							},
					},
				},

				#[wrap(Some)]
				set_content = &adw::NavigationPage {
					// This title isn't supposed to appear, but navigation pages want it
					// This will be overwritten by the title of its child's header bar
					set_title: "Content",

					#[wrap(Some)]
					set_child = model.content.widget(),
				},
			},

			connect_show[sender] => move |_| {
				sender.input(Self::Input::LoadPets);
			},

			connect_close_request[sender] => move |_| {
				sender.input(Self::Input::SavePets);
				glib::Propagation::Proceed
			},
		}
	}

	fn init(
		_init: Self::Init,
		window: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let pet_rows = FactoryVecDeque::<pet_row::Model>::builder()
			.launch_default()
			.detach();
		let content = content::Model::builder()
			.launch(content::Init { pet: None })
			.forward(sender.input_sender(), |response| match response {
				content::Output::AddPet(pet) => Self::Input::AddPetRow(pet),
				content::Output::RemoveSelectedPet => Self::Input::RemoveSelectedPetRow,
			});
		let model = Model {
			content,
			pet_rows,
			selected_row: None,
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
				let pets = self
					.pet_rows
					.iter()
					.map(|pet_row| Pet {
						name: pet_row.pet.borrow().name.clone(),
						gender: pet_row.pet.borrow().gender,
						species: pet_row.pet.borrow().species,
						birthdate: pet_row.pet.borrow().birthdate.clone(),
						was_sterilized: pet_row.pet.borrow().was_sterilized,
					})
					.collect::<Vec<Pet>>();

				let mut path = glib::user_data_dir();
				path.push(APP_ID);
				fs::create_dir_all(&path).expect("Should be able to create directory.");

				path.push(DATA_FILE_NAME);
				let file = fs::File::create(path).expect("Should be able to create YAML file.");

				serde_yml::to_writer(file, &pets)
					.expect("Should be able to write data to YAML file");
			}
			Self::Input::LoadPets => {
				let mut path = glib::user_data_dir();
				path.push(APP_ID);
				path.push(DATA_FILE_NAME);

				if let Ok(file) = fs::File::open(path) {
					let pets: Vec<Pet> = serde_yml::from_reader(file)
						.expect("Should be able to read data from YAML file.");

					for pet in pets {
						sender.input(Self::Input::AddPetRow(Rc::new(RefCell::new(pet))));
					}
				} else {
					// Assume it's the first time using the app.
				}
			}
			Self::Input::AddPetRow(pet) => {
				let index = self.pet_rows.guard().push_sorted(pet_row::Init { pet });
				self.selected_row = Some(index);
			}
			Self::Input::SelectPetRow(index) => {
				self.selected_row = Some(index);

				let selected_pet = &self.pet_rows[index].pet;
				self.content
					.sender()
					.send(content::Input::ShowPetDetails(Rc::clone(selected_pet)))
					.expect("Should be able to forward message to child");
			}
			Self::Input::RemoveSelectedPetRow => {
				if let Some(index) = self.selected_row {
					self.pet_rows.guard().remove(index);
					self.selected_row = None;
				}
			}
			Self::Input::ShowAddPetPane => {
				self.content
					.sender()
					.send(content::Input::ShowAddPetPane)
					.expect("Should be able to forward message to child");
			}
		}
	}

	fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
		Self::save_window_state(widgets);
	}
}
