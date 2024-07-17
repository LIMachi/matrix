use std::ops::{Add, Div, Mul};

use crate::prelude::{Norm, Vector};

impl <const S: usize, K: Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K> + Norm + Default + From<f32>> Vector<S, K> {
    ///I decided to make this function a method of Vector
    ///also, I made the type of return value K (since the angle might not be real with vectors of complex numbers)
    pub fn angle_cos(&self, v: &Self) -> K {
        self.dot(v) / K::from(self.norm() * v.norm())
    }
}