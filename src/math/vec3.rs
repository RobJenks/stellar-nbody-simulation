use std::ops::{Add, Mul};
use crate::core::types::Numeric;

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

    pub fn scale(&self, scalar: T) -> Self
        where T: Numeric + Mul<Output = T> {
        Self::new([
            self.data[0] * scalar,
            self.data[1] * scalar,
            self.data[2] * scalar
        ])
    }
}

impl <T> Clone for Vec3<T>
    where T: Numeric {

    fn clone(&self) -> Self {
        Vec3::new(self.data.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.data = source.data.clone();
    }
}

impl <T> Add for Vec3<T>
    where T: Numeric + Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        Self::Output::new([
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2] ])
    }
}
