use std::ops::Div;

use crate::prelude::Matrix;

impl <const C: usize, const R: usize, K: Div<Output = K> + Copy> Div<K> for Matrix<C, R, K> {
    type Output = Self;

    fn div(mut self, rhs: K) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] / rhs;
            }
        }
        self
    }
}