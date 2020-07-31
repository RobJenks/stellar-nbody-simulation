use std::ops::{Add, Mul, AddAssign};
use crate::core::types::{Numeric, Time};
use crate::state::State;

pub struct Integrator<TNum>
    where TNum: Numeric {
    _type_marker: TNum
}

impl <TNum> Integrator<TNum>
    where TNum: Numeric + Mul<Output=TNum> + AddAssign {

    pub fn integrate(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        self.integrate_acceleration(dt, state, result);
        self.integrate_velocity(dt, state, result);
    }

    fn integrate_acceleration(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        result.velocities_mut().iter_mut()
            .zip(state.accelerations())
            .for_each(|(vel, acc)| *vel += acc.scale(dt))
    }

    fn integrate_velocity(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        result.positions_mut().iter_mut()
            .zip(state.velocities())
            .for_each(|(pos, vel)| *pos += vel.scale(dt))
    }

}