use core::fmt::Debug;
use crate::core::types::*;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct State<TNum>
    where TNum: Numeric {

    id: Vec<String>,
    mass: Scalars<TNum>,
    position: Vectors<TNum>,
    velocity: Vectors<TNum>,
    acceleration: Vectors<TNum>,
}

impl <TNum> State<TNum>
    where TNum: Numeric {

    pub fn new() -> Self {
        Self {
            id: vec![],
            mass: vec![],
            position: vec![],
            velocity: vec![],
            acceleration: vec![]
        }
    }
pub fn set_id(&mut self, index: usize, id: String) { self.id[index] = id; }
    pub fn id(&self, index: usize) -> &String { &self.id[index] }
    pub fn ids(&self) -> &Vec<String> { &self.id }
    pub fn ids_mut(&mut self) -> &mut Vec<String> { &mut self.id }

    pub fn mass(&self, index: usize) -> TNum { self.mass[index] }
    pub fn masses(&self) -> &Scalars<TNum> { &self.mass }
    pub fn masses_mut(&mut self) -> &mut Scalars<TNum> { &mut self.mass }

    pub fn position(&self, index: usize) -> &Vec3<TNum> { &self.position[index] }
    pub fn positions(&self) -> &Vectors<TNum> { &self.position }
    pub fn positions_mut(&mut self) -> &mut Vectors<TNum> { &mut self.position }

    pub fn velocity(&self, index: usize) -> &Vec3<TNum> { &self.velocity[index] }
    pub fn velocities(&self) -> &Vectors<TNum> { &self.velocity }
    pub fn velocities_mut(&mut self) -> &mut Vectors<TNum> { &mut self.velocity }

    pub fn acceleration(&self, index: usize) -> &Vec3<TNum> { &self.acceleration[index] }
    pub fn accelerations(&self) -> &Vectors<TNum> { &self.acceleration }
    pub fn accelerations_mut(&mut self) -> &mut Vectors<TNum> { &mut self.acceleration }

    pub fn add_entity(&mut self, id: String, mass: TNum, position: Vec3<TNum>, velocity: Vec3<TNum>,
                      acceleration: Vec3<TNum>) {

        self.id.push(id);
        self.mass.push(mass);
        self.position.push(position);
        self.velocity.push(velocity);
        self.acceleration.push(acceleration);
    }
}

impl <TNum> Clone for State<TNum>
    where TNum: Numeric {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            mass: self.mass.clone(),
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone()
        }
    }
}