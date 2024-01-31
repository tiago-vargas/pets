use adw::prelude::*;
use relm4::prelude::*;

use std::{cell::RefCell, rc::Rc};
use super::pet::Pet;

pub(crate) struct Model {
    pet: Rc<RefCell<Pet>>,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
    SetPet(Rc<RefCell<Pet>>),
}

#[derive(Debug)]
pub(crate) enum Output {}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Input;
    type Output = Output;

    view! {
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
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
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
        }
    }
}
