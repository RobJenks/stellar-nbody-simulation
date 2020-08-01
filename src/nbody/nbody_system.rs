use std::ops::{Add, Sub, Mul, Div, AddAssign};
use core::iter::Sum;
use itertools::{Zip, multizip};
use ::core::cell::RefCell;
use std::ops::{Deref, DerefMut};
use crate::core::types::Numeric;
use crate::math::vec3::Vec3;
use crate::state::State;
use crate::core::types::*;
use crate::integrator::Integrator;
use crate::entities::system::System;
use failure::_core::cell::Ref;

pub struct NBodySystem<TNum>
    where TNum: Numeric {

    gravitational_constant: TNum,   // Gravitational constant G
    softening_constant: TNum,       // Compensates for Newtonian mechanics treating objects as point masses

    state_cycles: usize,
    current_state: usize,
    states: Vec<RefCell<State<TNum>>>,

    step_count: usize,
    integrator: Integrator<TNum>
}

impl<TNum> NBodySystem<TNum>
    where TNum: Numeric + Add<Output = TNum> + Sub<Output = TNum> + Mul<Output = TNum> + Div<Output = TNum> + AddAssign + Sum {

    pub fn new(system: &System, state_cycles: usize) -> Self {
        Self::new_from_params(
            TNum::from(system.get_gravitational_constant()),
            TNum::from(system.get_softeninig_constant()),
            system.generate_state(),
            state_cycles
        )
    }

    pub fn new_from_params(gravitational_constant: TNum, softening_constant: TNum,
                           initial_state: State<TNum>, state_cycles: usize) -> Self {
        Self {
            gravitational_constant,
            softening_constant,

            state_cycles,
            current_state: 0,
            states: NBodySystem::initialise_states(&initial_state, state_cycles),

            step_count: 0,
            integrator: Integrator::new()
        }
    }

    fn initialise_states(initial_state: &State<TNum>, state_cycles: usize) -> Vec<RefCell<State<TNum>>> {
        (0..state_cycles)
            .map(|_| initial_state.clone())
            .map(RefCell::new)
            .collect()
    }

    pub fn step(&mut self, dt: TNum) {
        {
            let state = self.states[self.current_state_index()].borrow();
            let mut next = self.states[self.next_state_index()].borrow_mut();

            self.step_for_states(dt, &*state, next.deref_mut());
        }

        self.advance_states();
        self.complete_step();
    }

    fn step_for_states(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        self.calculate_acceleration_systems(dt, state, result);
        self.integrator.integrate(dt, state, result);
    }

   pub fn current_state_index(&self) -> usize {
        self.current_state
    }

    fn next_state_index(&self) -> usize {
        self.determine_successor_state(self.current_state_index())
    }

    fn determine_predecessor_state(&self, current_state: usize, ) -> usize {
        if current_state == 0 { self.state_cycles - 1 } else { current_state - 1 }
    }

    fn determine_successor_state(&self, current_state: usize, ) -> usize {
        let next = current_state + 1;
        if next >= self.state_cycles { 0 } else { next }
    }

    fn advance_states(&mut self) {
        self.current_state = self.next_state_index();
    }

    fn complete_step(&mut self) {
        self.step_count += 1;
    }

    pub fn get_step_count(&self) -> usize {
        self.step_count
    }

    fn calculate_acceleration_systems(&self, dt: TNum, state: &State<TNum>, result: &mut State<TNum>) {
        result.accelerations_mut().iter_mut()
            .enumerate()
            .for_each(|(i, acc)| {
                let pos_i = state.position(i);
                *acc = state.masses().iter().zip(state.positions())
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, (mass_j, pos_j))| {
                        let d_pos = pos_j - pos_i;
                        let d_sq = d_pos.length_sq();

                        let force = (self.gravitational_constant * *mass_j) /
                            (d_sq * (d_sq + self.softening_constant).sq_root());

                        d_pos.scale(force)
                    })
                    .fold(Vec3::zero(), |sum, x| sum + x);
            });
    }

    pub fn get_current_state(&self) -> Ref<'_, State<TNum>> {
        self.states[self.current_state_index()].borrow()
    }

    // Returns 'state_count' states from the system history, up to a maximum of 'state_cycles' states
    pub fn get_state_history(&self, state_count: usize) -> Vec<Ref<'_, State<TNum>>> {
        self.states.iter()
            .rev()
            .cycle()
            .skip(self.state_cycles - self.current_state_index() - 1)
            .map(RefCell::borrow)
            .take(usize::min(state_count, self.state_cycles))
            .collect()
    }

}