use serde::*;
use crate::entities::entity::Entity;
use crate::core::types::Numeric;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entities {
    data: Vec<Entity>
}

impl Entities {
    pub fn get_data(&self) -> &Vec<Entity> { &self.data }
}

impl Clone for Entities {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone()
        }
    }
}