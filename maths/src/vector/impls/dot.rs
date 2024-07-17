use std::ops::{Add, Mul};

use crate::prelude::Vector;

impl <const S: usize, K: Add<Output = K> + Mul<Output = K> + Copy + Default> Vector<S, K> {
    pub fn dot(&self, other: &Self) -> K {
        let mut acc = K::default();
        for i in 0..S {
            acc = acc + self[i] * other[i];
        }
        acc
    }
}