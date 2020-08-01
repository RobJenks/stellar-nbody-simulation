use std::ops::{Add, Sub, Mul, AddAssign};
use core::iter::Sum;
use crate::core::types::Numeric;

#[derive(Debug)]
pub struct Vec3<T> {
    data: [T; 3]
}

impl <T> Vec3<T> {

    pub fn new(data: [T; 3]) -> Self {
        Self { data }
    }

    pub fn new_from_components(x: T, y: T, z: T) -> Self {
        Self { data: [x,y,z] }
    }

    pub fn zero() -> Self
        where T: Numeric {
        Self { data: [T::zero(); 3] }
    }

    pub fn add_vec(v0: &Self, v1: &Self) -> Self
        where T: Numeric + Add<Output = T> {

        Self::new([
            v0.data[0] + v1.data[0],
            v0.data[1] + v1.data[1],
            v0.data[2] + v1.data[2]
        ])
    }

    pub fn scale(&self, scalar: T) -> Self
        where T: Numeric + Mul<Output = T> {
        Self::new([
            self.data[0] * scalar,
            self.data[1] * scalar,
            self.data[2] * scalar
        ])
    }

    pub fn length(&self) -> T
        where T: Copy + Mul<Output = T> + Sum {
        self.data.iter().map(|&x| x * x).sum()
    }
}

impl <T> Clone for Vec3<T>
    where T: Numeric {

    fn clone(&self) -> Self {
        Vec3::new([
            self.data[0],
            self.data[1],
            self.data[2]
        ])
    }
}

impl <T> Add for Vec3<T>
    where T: Numeric + Add<Output = T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output
    {
        Self::Output::new([
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2] ])
    }
}

impl <T> AddAssign for Vec3<T>
    where T: Numeric + AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl <T> Sub for &Vec3<T>
    where T: Numeric + Sub<Output = T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output
    {
        Self::Output::new([
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2] ])
    }
}

impl <T> From<[f64; 3]> for Vec3<T>
    where T: From<f64> {

    fn from(x: [f64; 3]) -> Self {
        Self::new([T::from(x[0]), T::from(x[1]), T::from(x[2])])
    }
}