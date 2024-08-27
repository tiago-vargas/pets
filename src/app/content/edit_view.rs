use adw::prelude::*;
use relm4::prelude::*;

use super::Panes;

pub(crate) struct Model;

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Output {
	ApplyChanges,
	SetVisiblePane(Panes),
	DeletePet,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
	type Init = Init;
	type Input = ();
	type Output = Output;

	view! {
		adw::ToolbarView {
			add_top_bar = &adw::HeaderBar {
				pack_start = &gtk::Button {
					set_label: "Cancel",

					connect_clicked[sender] => move |_| {
						_ = sender.output(Self::Output::SetVisiblePane(Panes::PetDetails));
					},
				},

				#[wrap(Some)]
				set_title_widget = &gtk::Label {
					set_text: "Edit Pet",
					add_css_class: "heading",
				},

				pack_end = &gtk::Button {
					set_label: "Apply",
					add_css_class: "suggested-action",

					connect_clicked[sender] => move |_| {
						_ = sender.output(Self::Output::ApplyChanges);
					},
				},
			},

			#[wrap(Some)]
			set_content = &adw::Clamp {
				set_margin_all: 16,

				adw::StatusPage {
					set_title: "Edit View",

					#[wrap(Some)]
					set_child = &gtk::Button {
						set_label: "Delete",
						add_css_class: "destructive-action",

						connect_clicked => move |_this| {
							_ = sender.output(Self::Output::SetVisiblePane(Panes::NoPetSelected));
							_ = sender.output(Self::Output::DeletePet)
						},
					}
				},
			},
		}
	}

	fn init(
		_init: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self ;
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		let () = message;
	}
}
