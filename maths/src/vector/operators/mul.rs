use std::ops::{Mul, Add};

use crate::prelude::{Vector, Matrix};

impl <const S: usize, K: Mul<Output = K> + Copy> Mul for Vector<S, K> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> Mul<K> for Vector<S, K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>> Mul<Vector<R, K>> for Matrix<C, R, K> {
    type Output = Vector<C, K>;

    fn mul(self, rhs: Vector<R, K>) -> Self::Output {
        let mut out = Vector::<C, K>::default();
        for i in 0..R {
            for n in 0..C {
                out[i] = out[i] + rhs[n] * self[(n, i)];
            }
        }
        out
    }
}