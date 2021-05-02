use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stall {
    id: u8,
    name: String,
    category: u8,
}

impl Stall {
    pub fn category(&self) -> u8 {
        self.category
    }

    pub fn id(&self) -> u8 {
        self.id
    }
}
