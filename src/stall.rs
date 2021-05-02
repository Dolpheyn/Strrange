use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stall {
    id: u8,
    name: String,
    category: u8,
}

impl Stall {
    pub fn new(id: u8, name: &str, category: u8) -> Stall {
        Stall {
            id,
            name: name.to_string(),
            category,
        }
    }

    pub fn category(&self) -> u8 {
        self.category
    }
}
