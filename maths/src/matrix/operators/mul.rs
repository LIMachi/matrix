use std::ops::{Mul, Add};

use crate::prelude::Matrix;

impl <const C: usize, const R: usize, K: Mul<Output = K> + Copy> Mul<K> for Matrix<C, R, K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] * rhs;
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, const P: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>> Mul<Matrix<P, C, K>> for Matrix<C, R, K> {
    type Output = Matrix<P, R, K>;

    fn mul(self, rhs: Matrix<P, C, K>) -> Self::Output {
        let mut out = Matrix::<P, R, K>::default();
        for p in 0..P {
            for c in 0..C {
                for r in 0..R {
                    out[(p, r)] = out[(p, r)] + self[(c, r)] * rhs[(p, c)];
                }
            }
        }
        out
    }
}