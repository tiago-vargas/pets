use gtk::prelude::*;
use relm4::{factory::FactoryView, prelude::*};

use crate::app::pet::Pet;
use crate::app::AppInput;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct Model {
    pub(crate) pet: Rc<RefCell<Pet>>,
}

pub(crate) struct Init {
    pub(crate) pet: Rc<RefCell<Pet>>,
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for Model {
    type Init = Init;
    type Input = ();
    type Output = ();

    type CommandOutput = ();
    type ParentInput = AppInput;
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Label {
            set_text: &self.pet.borrow().name,
            set_halign: gtk::Align::Start,
        }
    }

    fn forward_to_parent(_output: Self::Output) -> Option<Self::ParentInput> {
        None
    }

    fn init_model(
        init: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { pet: init.pet }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, input: Self::Input, _sender: FactorySender<Self>) {
        let () = input;
    }
}

pub(crate) trait Sort {
    fn push_sorted(&mut self, pet_row: Init);
}

impl Sort for relm4::factory::FactoryVecDequeGuard<'_, Model> {
    fn push_sorted(&mut self, pet_row: Init) {
        let index = {
            let name = &pet_row.pet.borrow().name;

            let names = self
                .iter()
                .map(|row| String::from(&row.pet.borrow().name))
                .collect::<Vec<String>>();

            let names = names
                .iter()
                .map(|name| &name as &str)
                .collect::<Vec<&str>>();

            find_index_to_insert(&names, name)
        };

        self.insert(index, pet_row);
    }
}

fn find_index_to_insert(names: &[&str], name: &str) -> usize {
    match names.binary_search(&name) {
        // Found `name` in the list.
        // Insert here, nevertheless.
        // Won't overwrite the previous one, just shift it.
        Ok(index) => index,

        // Didn't find `name` in the list.
        // Insert here.
        Err(index) => index,
    }
}
