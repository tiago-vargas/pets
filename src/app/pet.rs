use serde::Serialize;

pub(crate) mod pet_row;

#[derive(Debug, Default, Serialize)]
pub(crate) struct Pet {
    pub(crate) name: String,
}
