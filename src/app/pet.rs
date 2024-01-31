pub(crate) mod pet_row;

#[derive(Debug)]
pub(crate) struct Pet {
    pub(crate) name: String,
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            name: String::from(""),
        }
    }
}
