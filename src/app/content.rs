use adw::prelude::*;
use relm4::prelude::*;

use crate::app::pet;
use std::{cell::RefCell, rc::Rc};

mod info_view;

pub(crate) struct ContentModel {
    selected_pet: Option<Rc<RefCell<pet::Pet>>>,
    is_adding_pet: bool,
    new_pet: Option<Rc<RefCell<pet::Pet>>>,
    info_view: Controller<info_view::Model>,
}

pub(crate) struct ContentInit {
    pub(crate) pet: Option<Rc<RefCell<pet::Pet>>>,
}

#[derive(Debug)]
pub(crate) enum ContentInput {
    ShowPet(Rc<RefCell<pet::Pet>>),
    ShowAddPetPane,
    UpdatePet(Rc<RefCell<pet::Pet>>),
    SendPetBack,
}

#[derive(Debug)]
pub(crate) enum ContentOutput {
    AddPet(Rc<RefCell<pet::Pet>>),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ContentInit;

    type Input = ContentInput;
    type Output = ContentOutput;

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
                                let new_pet = pet::Pet { name, ..pet::Pet::default() };
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

                                gtk::StackSwitcher {
                                    set_stack: Some(&stack)
                                },

                                #[name(stack)]
                                gtk::Stack {
                                    set_transition_type: gtk::StackTransitionType::Crossfade,

                                    add_named[Some("Info")] = model.info_view.widget(),
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
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            selected_pet: init.pet,
            is_adding_pet: false,
            new_pet: None,
            info_view: info_view::Model::builder()
                .launch(info_view::Init)
                .detach(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::ShowPet(pet) => {
                self.info_view.sender().send(info_view::Input::SetPet(Rc::clone(&pet)))
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
                        sender.input(Self::Input::ShowPet(Rc::clone(pet)));
                        self.info_view.sender().send(info_view::Input::SetPet(Rc::clone(pet)))
                            .expect("Should be able to send message to child");
                    }
                    None => (),
                }
            }
        }
    }
}
