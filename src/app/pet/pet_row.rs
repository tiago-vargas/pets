use gtk::prelude::*;
use relm4::{factory::FactoryView, prelude::*};

use crate::app::AppInput;
use crate::app::pet::Pet;

pub(crate) struct Model {
    pub(crate) pet: Pet,
}

pub(crate) struct Init {
    pub(crate) pet: Pet,
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
            set_text: &self.pet.name,
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
        match input {
            () => ()
        }
    }
}
