use std::ops::{Mul, Sub};
use crate::prelude::Vector;

impl <K: Mul<Output = K> + Sub<Output = K> + Copy> Vector<3, K> {
    pub fn cross_product(&self, v: &Self) -> Self {
        Vector::from([
            self.0[1] * v.0[2] - self.0[2] * v.0[1],
            self.0[2] * v.0[0] - self.0[0] * v.0[2],
            self.0[0] * v.0[1] - self.0[1] * v.0[0],
        ])
    }
}