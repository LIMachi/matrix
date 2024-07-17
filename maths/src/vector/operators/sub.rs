use std::ops::Sub;

use crate::prelude::{Vector, Matrix};

impl <const S: usize, K: Sub<Output = K> + Copy> Sub<K> for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs;
        }
        self
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Sub for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] - rhs[(c, r)];
            }
        }
        self
    }
}