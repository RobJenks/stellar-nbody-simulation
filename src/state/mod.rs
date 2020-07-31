use crate::core::types::*;
use crate::math::vec3::Vec3;


pub struct State<TNum> {
    mass: Scalars<TNum>,
    position: Vectors<TNum>,
    velocity: Vectors<TNum>,
    acceleration: Vectors<TNum>,
}

impl <TNum> State<TNum> {
    pub fn new() -> Self {
        Self {
            mass: vec![],
            position: vec![],
            velocity: vec![],
            acceleration: vec![]
        }
    }

    pub fn masses(&self) -> &Scalars<TNum> { &self.mass }
    pub fn masses_mut(&mut self) -> &mut Scalars<TNum> { &mut self.mass }

    pub fn positions(&self) -> &Vectors<TNum> { &self.position }
    pub fn positions_mut(&mut self) -> &mut Vectors<TNum> { &mut self.position }
    
    pub fn velocities(&self) -> &Vectors<TNum> { &self.velocity }
    pub fn velocities_mut(&mut self) -> &mut Vectors<TNum> { &mut self.velocity }
    
    pub fn accelerations(&self) -> &Vectors<TNum> { &self.acceleration }
    pub fn accelerations_mut(&mut self) -> &mut Vectors<TNum> { &mut self.acceleration }
}

impl <TNum> Clone for State<TNum>
    where TNum: Numeric {
    fn clone(&self) -> Self {
        Self {
            mass: self.mass.clone(),
            position: self.position.clone(),
            velocity: self.velocity.clone(),
            acceleration: self.acceleration.clone()
        }
    }
}