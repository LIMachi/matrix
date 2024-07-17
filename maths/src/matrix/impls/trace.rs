use std::ops::Add;

use crate::prelude::Matrix;

impl <const M: usize, K: Add<Output = K> + Default + Copy> Matrix<M, M, K> {
    pub fn trace(&self) -> K {
        let mut trace = K::default();
        for i in 0..M {
            trace = trace + self[(i, i)];
        }
        trace
    }
}