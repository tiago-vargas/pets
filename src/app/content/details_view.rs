use std::{cell::RefCell, rc::Rc};

use adw::prelude::*;
use relm4::prelude::*;

use crate::app::pet::Pet;

pub(crate) struct Model {
	pet: Rc<RefCell<Pet>>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	SetPet(Rc<RefCell<Pet>>),
	RedrawView,
}

#[derive(Debug)]
pub(crate) enum Output {
	ShowEditPet,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		adw::ToolbarView {
			add_top_bar = &adw::HeaderBar {
				#[wrap(Some)]
				set_title_widget = &gtk::Label {
					set_text: "Pet Details",
					add_css_class: "heading",
				},

				pack_end = &gtk::Button {
					set_label: "Edit",

					connect_clicked[sender] => move |_| {
						_ = sender.output(Self::Output::ShowEditPet);
					},
				},
			},

			#[wrap(Some)]
			set_content = &adw::Clamp {
				set_margin_all: 16,

				gtk::Box {
					set_orientation: gtk::Orientation::Vertical,
					set_spacing: 16,

					adw::Avatar {
						#[watch] set_text: Some(&model.pet.borrow().name),
						set_show_initials: true,
						set_size: 120,
					},

					gtk::Label {
						#[watch] set_text: &model.pet.borrow().name,
						set_css_classes: &["large-title"],
					},

					gtk::ListBox {
						set_css_classes: &["boxed-list"],

						adw::ActionRow {
							set_title: "Species",
							#[watch] set_subtitle: &model.pet.borrow().species.to_string(),

							add_css_class: "property",
						},

						adw::ActionRow {
							set_title: "Gender",
							#[watch] set_subtitle: &model.pet.borrow().gender.to_string(),

							add_css_class: "property",
						},

						adw::ActionRow {
							set_title: "Birthdate",
							#[watch] set_subtitle: &model.pet.borrow().birthdate.0.format("%d/%m/%Y")
								.expect("Format should exist"),

							add_css_class: "property",
						},

						adw::ActionRow {
							set_title: "Age",
							#[watch] set_subtitle: &model.pet.borrow().age(),

							add_css_class: "property",
						},

						adw::ActionRow {
							set_title: "Was spayed/neutered?",
							#[watch] set_subtitle: if model.pet.borrow().was_sterilized { "Yes" } else { "No" },

							add_css_class: "property",
						},
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
		let placeholder_pet = Pet::default();
		let model = Self {
			pet: Rc::new(RefCell::new(placeholder_pet)),
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Self::Input::SetPet(pet) => {
				self.pet = pet;
			}
			Self::Input::RedrawView => (),
		}
	}
}
