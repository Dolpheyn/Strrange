use serde::{Deserialize, Serialize};

pub type GivenStalls = Vec<Stall>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stall {
    pub id: u8,
    pub name: String,
    pub category: u8,
}

impl Stall {}

pub trait GetId {
    fn get_ids(&self) -> Vec<u8>;
}

impl GetId for GivenStalls {
    fn get_ids(&self) -> Vec<u8> {
        self.iter().map(|s| s.id).collect()
    }
}
