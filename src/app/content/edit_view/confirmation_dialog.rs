use adw::prelude::*;
use gtk::glib;
use relm4::prelude::*;

pub(crate) struct Model {
	pub(crate) is_presented: bool,
}

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {
	Present,
	Hide,
}

#[derive(Debug)]
pub(crate) enum Output {
	DeletePet,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = Input;
	type Output = Output;

	view! {
		adw::MessageDialog {
			set_heading: Some("Delete Pet?"),
			#[watch] set_visible: model.is_presented,

			add_response: ("no", "Cancel"),

			add_response: ("yes", "Delete"),
			set_response_appearance: ("yes", adw::ResponseAppearance::Destructive),

			connect_close_request[sender] => move |_this| {
				sender.input(Self::Input::Hide);
				glib::Propagation::Stop
			},

			connect_response: (Some("yes"), move |_this, _id| {
				_ = sender.output(Self::Output::DeletePet);
			}),
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			is_presented: false,
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			Self::Input::Present => self.is_presented = true,
			Self::Input::Hide => self.is_presented = false,
		}
	}
}
