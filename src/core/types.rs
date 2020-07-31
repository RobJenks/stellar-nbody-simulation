use std::ops::{Add, Mul};
use core::fmt::{Display, Debug};
use crate::math::vec3::Vec3;

pub type Time = f64;

pub type Scalars<TNum> = Vec<TNum>;
pub type Vectors<TNum> = Vec<Vec3<TNum>>;

pub trait Numeric
    where Self: Copy + Display + Debug + Mul + From<f64> {
    fn zero() -> Self;
    fn identity() -> Self;
    fn sq_root(&self) -> Self;
}

impl Numeric for f64 {
    fn zero() -> Self { 0.0 }
    fn identity() -> Self { 1.0 }
    fn sq_root(&self) -> Self { self.sqrt() }
}