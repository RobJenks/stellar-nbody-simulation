use std::ops::{Add, Sub, Mul, Div, AddAssign};
use core::iter::Sum;
use itertools::{Zip, multizip};
use crate::core::types::Numeric;
use crate::math::vec3::Vec3;
use crate::state::State;
use crate::core::types::*;
use crate::integrator::Integrator;
use crate::entities::system::System;

pub struct NBodySystem<TNum>
    where TNum: Numeric {

    gravitational_constant: TNum,   // Gravitational constant G
    softening_constant: TNum,       // Compensates for Newtonian mechanics treating objects as point masses

    integrator: Integrator<TNum>
}

impl<TNum> NBodySystem<TNum>
    where TNum: Numeric + Add<Output = TNum> + Sub<Output = TNum> + Mul<Output = TNum> + Div<Output = TNum> + AddAssign + Sum {

    pub fn new(system: &System) -> Self {
        Self::new_from_params(
            TNum::from(system.get_gravitational_constant()),
            TNum::from(system.get_softeninig_constant())
        )
    }

    pub fn new_from_params(gravitational_constant: TNum, softening_constant: TNum) -> Self {
        Self { gravitational_constant, softening_constant, integrator: Integrator::new() }
    }

    pub fn step(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        self.calculate_acceleration_systems(dt, state, result);
        self.integrator.integrate(dt, state, result);
    }

    fn calculate_acceleration_systems(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        result.accelerations_mut().iter_mut()
            .enumerate()
            .for_each(|(i, acc)| {
                let pos_i = state.position(i);
                *acc += state.masses().iter().zip(state.positions())
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, (mass_j, pos_j))| {
                        let d_pos = pos_j - pos_i;
                        let d_sq = d_pos.length();

                        let force = (self.gravitational_constant * *mass_j) /
                            (d_sq * (d_sq + self.softening_constant).sq_root());

                        d_pos.scale(force)
                    })
                    .fold(Vec3::zero(), |sum, x| sum + x)
                    .scale(dt);
            });
    }
}