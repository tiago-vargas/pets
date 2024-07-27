use adw::prelude::*;
use relm4::prelude::*;

use crate::app::pet;
use std::{cell::RefCell, rc::Rc};

pub(crate) struct Model {
	// You play around with this pet until you're satisfied with it.
	// Then create it, or dicard it.
	sandbox_pet: Rc<RefCell<pet::Pet>>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	Confirm,
	UpdatePet(Rc<RefCell<pet::Pet>>),
}

#[derive(Debug)]
pub(crate) enum Output {
	AddPet(Rc<RefCell<pet::Pet>>),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		gtk::Box {
			set_orientation: gtk::Orientation::Vertical,

			gtk::ListBox {
				set_margin_all: 16,
				add_css_class: "boxed-list",

				adw::EntryRow {
					set_title: "Pet Name",
					set_show_apply_button: true,

					connect_changed[sender] => move |entry| {
						let name = String::from(entry.text());
						let new_pet = pet::Pet { name, ..Default::default() };
						sender.input(Self::Input::UpdatePet(Rc::new(RefCell::new(new_pet))));
					},

					connect_apply[sender] => move |entry| {
						sender.input(Self::Input::Confirm);
						entry.set_text("");
					},

					connect_map => move |entry| {
						entry.grab_focus();
					},
				}
			}
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let default_pet = Rc::new(RefCell::new(pet::Pet::default()));
		let model = Self {
			sandbox_pet: default_pet,
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::UpdatePet(pet) => {
				self.sandbox_pet = Rc::clone(&pet);
			}
			Self::Input::Confirm => {
				sender
					.output(Self::Output::AddPet(Rc::clone(&self.sandbox_pet)))
					.expect("Should be able to send message to parent");
			}
		}
	}
}
