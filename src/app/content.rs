use std::{cell::RefCell, rc::Rc};

use relm4::prelude::*;

use crate::app::pet::Pet;

mod add_pet_view;
mod details_view;
mod edit_view;
mod no_pet_selected_view;

pub(crate) struct Model {
	selected_pet: Option<Rc<RefCell<Pet>>>,
	pub(crate) visible_pane: Panes,
	no_pet_selected_view: Controller<no_pet_selected_view::Model>,
	add_pet_view: Controller<add_pet_view::Model>,
	details_view: Controller<details_view::Model>,
	edit_view: Controller<edit_view::Model>,
}

pub(crate) struct Init {
	pub(crate) pet: Option<Rc<RefCell<Pet>>>,
}

#[derive(Debug)]
pub(crate) enum Input {
	ShowPetDetails(Rc<RefCell<Pet>>),
	ShowAddPetPane,
	SendPetBack(Rc<RefCell<Pet>>),
	SetVisiblePane(Panes),
	ApplyChanges,
	RemoveSelectedPet,
}

#[derive(Debug)]
pub(crate) enum Output {
	AddPet(Rc<RefCell<Pet>>),
	RemoveSelectedPet,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;

	type Input = Input;
	type Output = Output;

	view! {
		gtk::Stack {
			set_transition_type: gtk::StackTransitionType::Crossfade,

			add_named[Some(Panes::NoPetSelected.as_ref())] =
				model.no_pet_selected_view.widget(),
			add_named[Some(Panes::AddPet.as_ref())] =
				model.add_pet_view.widget(),
			add_named[Some(Panes::PetDetails.as_ref())] =
				model.details_view.widget(),
			add_named[Some(Panes::EditPet.as_ref())] =
				model.edit_view.widget(),

			#[watch] set_visible_child_name: model.visible_pane.as_ref(),
		}
	}

	fn init(
		init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			selected_pet: init.pet,
			visible_pane: Panes::NoPetSelected,
			no_pet_selected_view: no_pet_selected_view::Model::builder()
				.launch(no_pet_selected_view::Init)
				.detach(),
			add_pet_view: add_pet_view::Model::builder()
				.launch(add_pet_view::Init)
				.forward(sender.input_sender(), |output| match output {
					add_pet_view::Output::AddPet(pet) => Self::Input::SendPetBack(pet),
				}),
			details_view: details_view::Model::builder()
				.launch(details_view::Init)
				.forward(sender.input_sender(), |output| match output {
					details_view::Output::SetVisiblePane(pane) => Self::Input::SetVisiblePane(pane),
				}),
			edit_view: edit_view::Model::builder()
				.launch(edit_view::Init)
				.forward(sender.input_sender(), |output| match output {
					edit_view::Output::ApplyChanges => Self::Input::ApplyChanges,
					edit_view::Output::SetVisiblePane(pane) => Self::Input::SetVisiblePane(pane),
					edit_view::Output::DeletePet => Self::Input::RemoveSelectedPet,
				}),
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
				sender.input(Self::Input::SetVisiblePane(Panes::PetDetails));
				self.selected_pet = Some(pet);
			}
			Self::Input::ShowAddPetPane => {
				self.selected_pet = None;
				sender.input(Self::Input::SetVisiblePane(Panes::AddPet));
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
			Self::Input::RemoveSelectedPet => {
				self.selected_pet = None;
				_ = sender.output(Self::Output::RemoveSelectedPet);
			}
		}
	}
}

#[derive(Debug)]
pub(crate) enum Panes {
	NoPetSelected,
	AddPet,
	PetDetails,
	EditPet,
}

impl AsRef<str> for Panes {
	fn as_ref(&self) -> &str {
		match self {
			Panes::NoPetSelected => "No Pet Selected",
			Panes::AddPet => "Add Pet",
			Panes::PetDetails => "Pet Details",
			Panes::EditPet => "Edit Pet",
		}
	}
}
