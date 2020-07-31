use crate::core::types::{Numeric, Time};
use crate::state::State;

pub struct Integrator<TNum>
    where TNum: Numeric {
    _type_marker: TNum
}

impl <TNum> Integrator<TNum>
    where TNum: Numeric {

    pub fn integrate(&self, t: Time, state: &State<TNum>, result: &mut State<TNum>) {

    }

}