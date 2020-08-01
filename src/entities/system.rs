use serde::*;
use super::entity::Entity;
use crate::core::types::Numeric;
use serde::Deserialize;
use crate::state::State;
use crate::math::vec3::Vec3;
use failure::_core::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    id: String,

    gravitational_constant: f64,
    softening_constant: f64,

    entities: Vec<Entity>
}

impl System {
    pub fn get_gravitational_constant(&self) -> f64 { self.gravitational_constant }
    pub fn get_softening_constant(&self) -> f64 { self.softening_constant }

    pub fn from_file(file: &str) -> Self {
        let data = std::fs::read_to_string(file)
            .expect(format!("Failed to read file ({})", file).as_str());

        serde_json::from_str(data.as_str())
            .expect(format!("Failed to deserialize data from ({})", file).as_str())
    }

    pub fn generate_state<TNum>(&self) -> State<TNum>
        where TNum: Numeric {

        let mut state = State::<TNum>::new();

        self.entities.iter().for_each(|x| state.add_entity(
            x.id.clone(),
            TNum::from_f64(x.mass),
            Vec3::from(x.position),
            Vec3::from(x.velocity),
            Vec3::from(x.acceleration)));

        state
    }
}

impl Clone for System {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            gravitational_constant: self.gravitational_constant.clone(),
            softening_constant: self.softening_constant.clone(),
            entities: self.entities.clone()
        }
    }

}