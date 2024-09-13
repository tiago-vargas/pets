use std::{cell::RefCell, rc::Rc};

use adw::prelude::*;
use gtk::glib;
use relm4::prelude::*;

use crate::app::pet::{ComboRow, Gender, Pet, Species};

mod confirmation_dialog;

pub(crate) struct Model {
	original_pet: Rc<RefCell<Pet>>,
	// You play around with this pet until you're satisfied with it.
	// Then update the original with it, or dicard it.
	sandbox_pet: Pet,
	confirmation_dialog: Controller<confirmation_dialog::Model>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	SetPet(Rc<RefCell<Pet>>),
	ApplyChanges,
	DiscardChanges,
	SetName(String),
	SetSpecies(u32),
	SetGender(u32),
	SetBirthdate(glib::DateTime),
	SetWasSterilized(u32),
	ShowConfirmationDialog,
	DeletePet,
}

#[derive(Debug)]
pub(crate) enum Output {
	DismissPane,
	DeletePet,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		adw::ToolbarView {
			add_top_bar = &adw::HeaderBar {
				pack_start = &gtk::Button {
					set_label: "Cancel",

					connect_clicked[sender] => move |_| {
						_ = sender.output(Self::Output::DismissPane);
					},
				},

				#[wrap(Some)]
				set_title_widget = &gtk::Label {
					set_text: "Edit Pet",
					add_css_class: "heading",
				},

				pack_end = &gtk::Button {
					set_label: "Apply",
					add_css_class: "suggested-action",

					connect_clicked[sender] => move |_| {
						sender.input(Self::Input::ApplyChanges);
					},
				},
			},

			#[wrap(Some)]
			set_content = &adw::Clamp {
				set_margin_all: 16,

				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_spacing: 16,

					gtk::ListBox {
						add_css_class: "boxed-list",

						#[name(name_entry)]
						adw::EntryRow {
							set_title: "Name",

							#[track = "name_entry.text() != model.sandbox_pet.name"]
							set_text: &model.sandbox_pet.name,

							connect_changed[sender] => move |this| {
								let name = String::from(this.text());
								sender.input(Self::Input::SetName(name));
							},
						},

						adw::ComboRow {
							set_title: "Species",

							set_model: Some(&Species::list()),
							#[watch] set_selected: model.sandbox_pet.species as u32,

							connect_selected_notify[sender] => move |this| {
								sender.input(Self::Input::SetSpecies(this.selected()));
							},
						},

						adw::ComboRow {
							set_title: "Gender",

							set_model: Some(&Gender::list()),
							#[watch] set_selected: model.sandbox_pet.gender as u32,

							connect_selected_notify[sender] => move |this| {
								sender.input(Self::Input::SetGender(this.selected()));
							},
						},

						adw::ActionRow {
							set_title: "Birthdate",

							add_suffix = &gtk::MenuButton {
								#[watch] set_label: &model.sandbox_pet.birthdate.0.format("%d/%m/%Y")
									.expect("Format should exist"),

								set_valign: gtk::Align::Center,

								#[wrap(Some)]
								set_popover = &gtk::Popover {
									gtk::Calendar {
										#[watch] set_day: model.sandbox_pet.birthdate.0.day_of_month(),
										#[watch] set_month: model.sandbox_pet.birthdate.0.month() - 1,
										#[watch] set_year: model.sandbox_pet.birthdate.0.year(),

										connect_day_selected[sender] => move |this| {
											sender.input(Self::Input::SetBirthdate(this.date()))
										},
									},
								},
							},
						},

						adw::ComboRow {
							set_title: "Was spayed/neutered?",

							set_model: Some(&gtk::StringList::new(&["No", "Yes"])),
							#[watch] set_selected: model.sandbox_pet.was_sterilized as u32,

							connect_selected_notify[sender] => move |this| {
								sender.input(Self::Input::SetWasSterilized(this.selected()));
							},
						},
					},

					gtk::Button {
						set_label: "Delete",
						add_css_class: "destructive-action",

						connect_clicked[sender] => move |_this| {
							sender.input(Self::Input::ShowConfirmationDialog);
						},
					},

					connect_unmap[sender] => move |_| {
						sender.input(Self::Input::DiscardChanges);
					},
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let placeholder_pet_1 = Pet::default();
		let placeholder_pet_2 = Pet::default();
		let model = Self {
			original_pet: Rc::new(RefCell::new(placeholder_pet_1)),
			sandbox_pet: placeholder_pet_2,
			confirmation_dialog: confirmation_dialog::Model::builder()
				.transient_for(&root)
				.launch(confirmation_dialog::Init)
				.forward(sender.input_sender(), |output| match output {
					confirmation_dialog::Output::DeletePet => Self::Input::DeletePet,
				}),
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::SetPet(pet) => {
				self.sandbox_pet = Pet::from(&pet);
				self.original_pet = pet;
			}
			Self::Input::ApplyChanges => {
				self.original_pet.borrow_mut().name = self.sandbox_pet.name.clone();
				self.original_pet.borrow_mut().gender = self.sandbox_pet.gender;
				self.original_pet.borrow_mut().species = self.sandbox_pet.species;
				self.original_pet.borrow_mut().birthdate = self.sandbox_pet.birthdate.clone();
				self.original_pet.borrow_mut().was_sterilized = self.sandbox_pet.was_sterilized;

				_ = sender.output(Self::Output::DismissPane);
			}
			Self::Input::DiscardChanges => {
				self.sandbox_pet = Pet::default();
			}
			Self::Input::SetName(name) => {
				self.sandbox_pet.name = name;
			}
			Self::Input::SetSpecies(index) => {
				self.sandbox_pet.species =
					Species::try_from(index).expect("Index from list row should be valid");
			}
			Self::Input::SetGender(index) => {
				self.sandbox_pet.gender =
					Gender::try_from(index).expect("Index from list row should be valid");
			}
			Self::Input::SetBirthdate(date) => {
				self.sandbox_pet.birthdate.0 = date;
			}
			Self::Input::SetWasSterilized(index) => {
				self.sandbox_pet.was_sterilized = match index {
					0 => false,
					1 => true,
					_ => unreachable!("Index is too large"),
				};
			}
			Self::Input::ShowConfirmationDialog => {
				_ = self
					.confirmation_dialog
					.sender()
					.send(confirmation_dialog::Input::Present);
			}
			Self::Input::DeletePet => {
				_ = sender.output(Self::Output::DeletePet);
			}
		}
	}
}
