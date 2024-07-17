use std::ops::Add;

use crate::prelude::{Vector, Matrix};

impl <const S: usize, K: Add<Output = K> + Copy> Add<K> for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs;
        }
        self
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> Add for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] + rhs[(c, r)];
            }
        }
        self
    }
}