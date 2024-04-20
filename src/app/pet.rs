use gtk::glib;
use relm4::prelude::*;

use serde::{Deserialize, Serialize};

pub(crate) mod pet_row;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Pet {
	pub(crate) name: String,
	pub(crate) gender: Gender,
	pub(crate) species: Species,
	pub(crate) birthdate: Birthdate,
	pub(crate) was_sterilized: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) enum Gender {
	#[default]
	Male,
	Female,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) enum Species {
	#[default]
	Cat,
	Dog,
}

/// A wrapper around `glib::DateTime` to allow custom serialization and
/// deserialization, while obeying the orphan rules.
#[derive(Clone, Debug)]
pub(crate) struct Birthdate(pub(crate) glib::DateTime);

impl Serialize for Birthdate {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let timestamp = self.0.to_unix();
		serializer.serialize_i64(timestamp)
	}
}

impl<'de> Deserialize<'de> for Birthdate {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let timestamp = i64::deserialize(deserializer)?;
		match glib::DateTime::from_unix_local(timestamp) {
			Ok(datetime) => Ok(Birthdate(datetime)),
			Err(e) => Err(serde::de::Error::custom(e.to_string())),
		}
	}
}

impl Default for Birthdate {
	fn default() -> Self {
		Self(glib::DateTime::now_local().expect("Should be able to get current date and time"))
	}
}
