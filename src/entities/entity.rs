use serde::*;
use crate::core::types::Numeric;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub id: String,
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub acceleration: [f64; 3]
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            mass: self.mass,
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration
        }
    }
}