use adw::prelude::*;
use relm4::prelude::*;

use crate::app::pet;
use std::{cell::RefCell, rc::Rc};

mod add_pet_view;
mod details_view;
mod edit_view;

pub(crate) struct Model {
	selected_pet: Option<Rc<RefCell<pet::Pet>>>,
	is_adding_pet: bool,
	visible_pane: Panes,
	add_pet_view: Controller<add_pet_view::Model>,
	details_view: Controller<details_view::Model>,
	edit_view: Controller<edit_view::Model>,
}

pub(crate) struct Init {
	pub(crate) pet: Option<Rc<RefCell<pet::Pet>>>,
}

#[derive(Debug)]
pub(crate) enum Input {
	ShowPetDetails(Rc<RefCell<pet::Pet>>),
	ShowAddPetPane,
	SendPetBack(Rc<RefCell<pet::Pet>>),
	SetVisiblePane(Panes),
	ApplyChanges,
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
		adw::Bin {  // `if` needs an outer widget
			if model.is_adding_pet {
				adw::Bin {
					#[wrap(Some)]
					set_child = model.add_pet_view.widget(),
				}
			} else {
				adw::Bin {
					match &model.selected_pet {  // `match` needs an outer widget
						None => adw::StatusPage {
							set_title: "No Pet Selected",
						}
						Some(_) => &adw::Clamp {
							set_margin_all: 16,

							gtk::Box {
								set_orientation: gtk::Orientation::Vertical,
								set_spacing: 16,

								gtk::Stack {
									set_transition_type: gtk::StackTransitionType::Crossfade,
									#[watch] set_visible_child_name: model.visible_pane.as_ref(),

									add_named[Some(Panes::PetDetails.as_ref())] =
										model.details_view.widget(),
									add_named[Some(Panes::EditPet.as_ref())] =
										model.edit_view.widget(),
								}
							}
						}
					}
				}
			}
		}
	}

	fn init(
		init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			selected_pet: init.pet,
			is_adding_pet: false,
			visible_pane: Panes::PetDetails,
			add_pet_view: add_pet_view::Model::builder()
				.launch(add_pet_view::Init)
				.forward(sender.input_sender(), |output| match output {
					add_pet_view::Output::AddPet(pet) => Self::Input::SendPetBack(pet),
				}),
			details_view: details_view::Model::builder()
				.launch(details_view::Init)
				.detach(),
			edit_view: edit_view::Model::builder()
				.launch(edit_view::Init)
				.detach(),
			};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
		match message {
			Self::Input::ShowPetDetails(pet) => {
				self.details_view
					.sender()
					.send(details_view::Input::SetPet(Rc::clone(&pet)))
					.expect("Should be able to send message to child");
				self.selected_pet = Some(pet);
				self.is_adding_pet = false;
			}
			Self::Input::ShowAddPetPane => {
				self.selected_pet = None;
				self.is_adding_pet = true;
			}
			Self::Input::SendPetBack(pet) => {
				// This is a workaround to avoid moving the RC out of `self` in `view!`
				// using `output` directly.
				sender
					.output(Self::Output::AddPet(Rc::clone(&pet)))
					.expect("Should be able to send message to parent");
				sender.input(Self::Input::ShowPetDetails(Rc::clone(&pet)));
				self.details_view
					.sender()
					.send(details_view::Input::SetPet(Rc::clone(&pet)))
					.expect("Should be able to send message to child");
			}
			Self::Input::SetVisiblePane(pane) => {
				self.visible_pane = pane;
			}
			Self::Input::ApplyChanges => {
				sender.input(Self::Input::SetVisiblePane(Panes::PetDetails));
			}
		}
	}
}

#[derive(Debug)]
pub(crate) enum Panes {
	PetDetails,
	EditPet,
}

impl AsRef<str> for Panes {
	fn as_ref(&self) -> &str {
		match self {
			Panes::PetDetails => "Pet Details",
			Panes::EditPet => "Edit Pet",
		}
	}
}
