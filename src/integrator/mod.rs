use std::ops::{Add, Mul, AddAssign};
use crate::core::types::{Numeric, Time};
use crate::state::State;
use core::marker::PhantomData;
use crate::math::vec3::Vec3;

pub struct Integrator<TNum>
    where TNum: Numeric {
    _type_marker: PhantomData<TNum>
}

impl <TNum> Integrator<TNum>
    where TNum: Numeric + Add<Output=TNum> + Mul<Output=TNum> + AddAssign {

    pub fn new() -> Self { Self { _type_marker: PhantomData } }

    pub fn integrate(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        self.integrate_acceleration(dt, state, result);
        self.integrate_velocity(dt, state, result);
    }

    fn integrate_acceleration(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        result.velocities_mut().iter_mut()
            .zip(state.velocities().iter().zip(state.accelerations()))
            .for_each(|(new_vel, (vel, acc))|
                *new_vel = Vec3::add_vec(vel, &acc.scale(dt)));
    }

    fn integrate_velocity(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        result.positions_mut().iter_mut()
            .zip(state.positions().iter().zip(state.velocities()))
            .for_each(|(new_pos, (pos, vel))|
                *new_pos = Vec3::add_vec(pos, &vel.scale(dt)));
    }

}