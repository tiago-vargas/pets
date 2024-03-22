use adw::prelude::*;
use relm4::prelude::*;

use crate::app::pet;
use std::rc::Rc;

pub(crate) struct ContentModel {
    selected_pet: Option<Rc<pet::Pet>>,
}

pub(crate) struct ContentInit {
    pub(crate) pet: Option<Rc<pet::Pet>>,
}

#[derive(Debug)]
pub(crate) enum ContentInput {
    ShowPet(Rc<pet::Pet>),
}

#[derive(Debug)]
pub(crate) enum ContentOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ContentInit;

    type Input = ContentInput;
    type Output = ContentOutput;

    view! {
        #[root]
        adw::Bin {
            match &model.selected_pet {
                None => adw::StatusPage {
                    set_title: "No Pet Selected",
                }
                Some(pet) => &gtk::Label {
                    #[watch] set_label: &pet.name,
                    set_margin_all: 4,
                    set_css_classes: &["title-1"],
                    set_vexpand: true,
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            selected_pet: init.pet,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::ShowPet(pet) => {
                self.selected_pet = Some(pet);
            }
        }
    }
}
