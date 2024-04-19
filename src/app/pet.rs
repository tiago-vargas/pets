use serde::{Deserialize, Serialize};

pub(crate) mod pet_row;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Pet {
    pub(crate) name: String,
}
