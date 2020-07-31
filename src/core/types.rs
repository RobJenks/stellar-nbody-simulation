use crate::math::vec3::Vec3;

pub type Time = f64;

pub type Scalars<TNum> = Vec<TNum>;
pub type Vectors<TNum> = Vec<Vec3<TNum>>;

pub trait Numeric: Copy {
    fn zero() -> Self;
    fn identity() -> Self;
}

impl Numeric for f64 {
    fn zero() -> Self { 0.0 }
    fn identity() -> Self { 1.0 }
}

