use std::ops::{Add, Mul};
use core::fmt::{Display, Debug};
use crate::math::vec3::Vec3;
use fixed::{FixedI64, FixedI128};
use fixed::types::{I16F48, I64F64};

pub type Time = f64;

pub type Scalars<TNum> = Vec<TNum>;
pub type Vectors<TNum> = Vec<Vec3<TNum>>;

pub trait Numeric
    where Self: Copy + Display + Debug + Mul {
    fn zero() -> Self;
    fn identity() -> Self;
    fn sq_root(&self) -> Self;
    fn from_f64(x: f64) -> Self;
    fn into_f64(self) -> f64;
}

impl Numeric for f64 {
    fn zero() -> Self { 0.0 }
    fn identity() -> Self { 1.0 }
    fn sq_root(&self) -> Self { self.sqrt() }
    fn from_f64(x: f64) -> Self { x }
    fn into_f64(self) -> f64 { self }
}

impl Numeric for I16F48 {
    fn zero() -> Self { Numeric::from_f64(0.0) }
    fn identity() -> Self { Numeric::from_f64(1.0) }
    fn sq_root(&self) -> Self {
        Numeric::from_f64(self.into_f64().sqrt())
    }
    fn from_f64(x: f64) -> Self { FixedI64::from_num::<f64>(x) }
    fn into_f64(self) -> f64 { self.to_num::<f64>() }
}

impl Numeric for I64F64 {
    fn zero() -> Self { Numeric::from_f64(0.0) }
    fn identity() -> Self { Numeric::from_f64(1.0) }
    fn sq_root(&self) -> Self {
        Numeric::from_f64(self.into_f64().sqrt())
    }
    fn from_f64(x: f64) -> Self { FixedI128::from_num::<f64>(x) }
    fn into_f64(self) -> f64 { self.to_num::<f64>() }
}
