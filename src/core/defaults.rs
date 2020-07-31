use crate::core::types::*;
use failure::_core::marker::PhantomData;
use crate::math::vec3::Vec3;

pub struct Defaults<TNum> where TNum: Numeric {
    _type_marker : PhantomData<TNum>
}

impl <TNum> Defaults<TNum> where TNum: Numeric {
    pub fn zero_vec() -> Vec3<TNum> { Vec3::new([TNum::zero(); 3]) }

    pub fn default_mass() -> TNum { TNum::identity() }
    pub fn default_position() -> Vec3<TNum> { Self::zero_vec() }
    pub fn default_velocity() -> Vec3<TNum> { Self::zero_vec() }
    pub fn default_acceleration() -> Vec3<TNum> { Self::zero_vec() }

}