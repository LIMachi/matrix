use std::ops::Sub;

use crate::prelude::{Matrix, Vector};

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<K> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] - rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<Vector<C, K>> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: Vector<C, K>) -> Self::Output {
        for r in 0..R {
            self[r] = self[r] - rhs;
        }
        self
    }
}