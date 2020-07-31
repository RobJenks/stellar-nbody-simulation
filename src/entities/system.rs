use serde::*;
use super::entities::Entities;
use crate::core::types::Numeric;
use serde::Deserialize;
use crate::state::State;
use crate::math::vec3::Vec3;
use failure::_core::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct System<TNum>
    where TNum: Numeric {
    _type_marker: PhantomData<TNum>,

    id: String,

    gravitational_constant: f64,
    softening_constant: f64,

    entities: Entities
}

impl <TNum> System<TNum>
    where TNum: Numeric {

    pub fn from_file(file: &str) -> Self {
        let data = std::fs::read_to_string(file)
            .expect(format!("Failed to read file ({})", file).as_str());

        serde_json::from_str(data.as_str())
            .expect(format!("Failed to deserialize data from ({})", file).as_str())
    }

    pub fn generate_state(&self) -> State<TNum> {
        let mut state = State::<TNum>::new();

        self.entities.get_data().iter().for_each(|x| state.add_entity(
            x.id.clone(),
            TNum::from(x.mass),
            Vec3::from(x.position),
            Vec3::from(x.velocity),
            Vec3::from(x.acceleration)));

        state
    }
}

impl <TNum> Clone for System<TNum>
    where TNum: Clone + Numeric {

    fn clone(&self) -> Self {
        Self {
            _type_marker: PhantomData,
            id: self.id.clone(),
            gravitational_constant: self.gravitational_constant.clone(),
            softening_constant: self.softening_constant.clone(),
            entities: self.entities.clone()
        }
    }

}