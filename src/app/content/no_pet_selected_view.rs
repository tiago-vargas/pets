use adw::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model;

pub(crate) struct Init;

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = ();

	view! {
		adw::ToolbarView {
			add_top_bar = &adw::HeaderBar {
				#[wrap(Some)]
				set_title_widget = &gtk::Label {
					set_text: "Pet Details",
					add_css_class: "heading",
				},
			},

			#[wrap(Some)]
			set_content = &adw::Clamp {
				set_margin_all: 16,

				adw::StatusPage {
					set_title: "No Pet Selected",
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self;
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}
}
