use adw::prelude::*;
use gtk::glib;
use relm4::prelude::*;

use crate::app::pet::{ComboRow, Gender, Pet, Species};
use std::{cell::RefCell, rc::Rc};

pub(crate) struct Model {
	// You play around with this pet until you're satisfied with it.
	// Then create it, or dicard it.
	sandbox_pet: Rc<RefCell<Pet>>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	Confirm,
	Discard,
	SetName(String),
	SetSpecies(u32),
	SetGender(u32),
	SetBirthdate(glib::DateTime),
	SetWasSterilized(u32),
}

#[derive(Debug)]
pub(crate) enum Output {
	AddPet(Rc<RefCell<Pet>>),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,
			set_spacing: 16,

			gtk::ListBox {
				add_css_class: "boxed-list",

				adw::EntryRow {
					set_title: "Name",

					connect_changed[sender] => move |entry| {
						let name = String::from(entry.text());
						sender.input(Self::Input::SetName(name));
					},

					connect_map => move |entry| {
						entry.grab_focus();
						entry.set_text("");
					},
				},

				adw::ComboRow {
					set_title: "Species",

					set_model: Some(&Species::list()),
					#[watch] set_selected: model.sandbox_pet.borrow().species as u32,

					connect_selected_notify[sender] => move |this| {
						sender.input(Self::Input::SetSpecies(this.selected()));
					},
				},

				adw::ComboRow {
					set_title: "Gender",

					set_model: Some(&Gender::list()),
					#[watch] set_selected: model.sandbox_pet.borrow().gender as u32,

					connect_selected_notify[sender] => move |this| {
						sender.input(Self::Input::SetGender(this.selected()));
					},
				},

				adw::ActionRow {
					set_title: "Birthdate",

					add_suffix = &gtk::MenuButton {
						#[watch] set_label: &model.sandbox_pet.borrow().birthdate.0.format("%d/%m/%Y")
							.expect("Format should exist"),

						set_valign: gtk::Align::Center,

						#[wrap(Some)]
						set_popover = &gtk::Popover {
							gtk::Calendar {
								#[watch] set_day: model.sandbox_pet.borrow().birthdate.0.day_of_month(),
								#[watch] set_month: model.sandbox_pet.borrow().birthdate.0.month() - 1,
								#[watch] set_year: model.sandbox_pet.borrow().birthdate.0.year(),

								connect_day_selected[sender] => move |calendar| {
									sender.input(Self::Input::SetBirthdate(calendar.date()))
								},
							},
						},
					},
				},

				adw::ComboRow {
					set_title: "Was spayed/neutered?",

					set_model: Some(&gtk::StringList::new(&["No", "Yes"])),
					#[watch] set_selected: model.sandbox_pet.borrow().was_sterilized as u32,

					connect_selected_notify[sender] => move |combo_row| {
						sender.input(Self::Input::SetWasSterilized(combo_row.selected()));
					},
				},
			},

			gtk::Button {
				set_label: "Apply",
				add_css_class: "suggested-action",

				connect_clicked[sender] => move |_| {
					sender.input(Self::Input::Confirm);
				},
			},

			connect_unmap => move |_| {
				sender.input(Self::Input::Discard);
			},
		},
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let default_pet = Rc::new(RefCell::new(Pet::default()));
		let model = Self {
			sandbox_pet: default_pet,
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::Confirm => {
				sender
					.output(Self::Output::AddPet(Rc::clone(&self.sandbox_pet)))
					.expect("Should be able to send message to parent");
			}
			Self::Input::Discard => {
				let default_pet = Pet::default();
				self.sandbox_pet = Rc::new(RefCell::new(default_pet));
			}
			Self::Input::SetName(name) => {
				self.sandbox_pet.borrow_mut().name = name;
			}
			Self::Input::SetSpecies(index) => {
				self.sandbox_pet.borrow_mut().species =
					Species::try_from(index).expect("Index from list row should be valid");
			}
			Self::Input::SetGender(index) => {
				self.sandbox_pet.borrow_mut().gender =
					Gender::try_from(index).expect("Index from list row should be valid");
			}
			Self::Input::SetBirthdate(date) => {
				self.sandbox_pet.borrow_mut().birthdate.0 = date;
			}
			Self::Input::SetWasSterilized(index) => {
				self.sandbox_pet.borrow_mut().was_sterilized = match index {
					0 => false,
					1 => true,
					_ => unreachable!("Index is too large"),
				};
			}
		}
	}
}
