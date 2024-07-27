use std::fmt;

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

impl fmt::Display for Gender {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Gender::Male => write!(f, "Male"),
			Gender::Female => write!(f, "Female"),
		}
	}
}

impl ComboRow for Gender {
	fn list() -> gtk::StringList {
		gtk::StringList::new(&[&Self::Male.to_string(), &Self::Female.to_string()])
	}
}

impl TryFrom<u32> for Gender {
	type Error = ();

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Male),
			1 => Ok(Self::Female),
			_ => unreachable!("Index is too large"),
		}
	}
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub(crate) enum Species {
	#[default]
	Cat,
	Dog,
}

impl fmt::Display for Species {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Cat => write!(f, "Cat"),
			Self::Dog => write!(f, "Dog"),
		}
	}
}

impl ComboRow for Species {
	fn list() -> gtk::StringList {
		gtk::StringList::new(&[&Self::Cat.to_string(), &Self::Dog.to_string()])
	}
}

impl TryFrom<u32> for Species {
	type Error = ();

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(Self::Cat),
			1 => Ok(Self::Dog),
			_ => unreachable!("Index is too large"),
		}
	}
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

impl Pet {
	pub(crate) fn age(&self) -> String {
		let now = glib::DateTime::now_local().expect("Should be able to get current date and time");

		struct Age {
			year: i32,
			month: i32,
			day: i32,
		}

		let age = Age {
			year: now.year() - self.birthdate.0.year(),
			month: now.month() - self.birthdate.0.month(),
			day: now.day_of_month() - self.birthdate.0.day_of_month(),
		};

		match age {
			Age { year: 0, month: 0, day: 1} => String::from("1 day"),
			Age { year: 0, month: 0, day} => format!("{day} days"),
			Age { year: 0, month: 1, ..} => String::from("1 month"),
			Age { year: 0, month, ..} => format!("{month} months"),
			Age { year: 1, .. } => String::from("1 year"),
			Age { year, .. } => format!("{year} years"),
		}
	}
}

pub(crate) trait ComboRow {
	fn list() -> gtk::StringList;
}
