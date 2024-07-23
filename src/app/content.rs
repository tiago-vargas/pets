use adw::prelude::*;
use relm4::prelude::*;

use crate::app::pet;
use std::{cell::RefCell, rc::Rc};

mod details_view;
mod edit_view;

pub(crate) struct Model {
	selected_pet: Option<Rc<RefCell<pet::Pet>>>,
	is_adding_pet: bool,
	new_pet: Option<Rc<RefCell<pet::Pet>>>,
	visible_pane: Panes,
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
	UpdatePet(Rc<RefCell<pet::Pet>>),
	SendPetBack,
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
		#[root]
		adw::Bin {  // `if` needs an outer widget
			if model.is_adding_pet {
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
								let new_pet = pet::Pet { name };
								sender.input(Self::Input::UpdatePet(Rc::new(RefCell::new(new_pet))));
							},

							connect_apply[sender] => move |entry| {
								sender.input(Self::Input::SendPetBack);
								entry.set_text("");
							},

							connect_map => move |entry| {
								entry.grab_focus();
							},
						}
					}
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
			new_pet: None,
			visible_pane: Panes::PetDetails,
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
				self.details_view.sender().send(details_view::Input::SetPet(Rc::clone(&pet)))
					.expect("Should be able to send message to child");
				self.selected_pet = Some(pet);
				self.is_adding_pet = false;
			}
			Self::Input::ShowAddPetPane => {
				self.selected_pet = None;
				self.is_adding_pet = true;
			}
			Self::Input::UpdatePet(pet) => {
				self.new_pet = Some(Rc::clone(&pet));
			}
			Self::Input::SendPetBack => {
				// This is a workaround to avoid moving the RC out of `self` in `view!`
				// using `output` directly.
				match &self.new_pet {
					Some(pet) => {
						sender.output(Self::Output::AddPet(Rc::clone(pet)))
							.expect("Should be able to send message to parent");
						sender.input(Self::Input::ShowPetDetails(Rc::clone(pet)));
						self.details_view.sender().send(details_view::Input::SetPet(Rc::clone(pet)))
							.expect("Should be able to send message to child");
					}
					None => (),
				}
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
