use std::ops::Div;

use crate::prelude::Vector;

impl <const S: usize, K: Div<Output = K> + Copy> Div for Vector<S, K> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] / rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> Div<K> for Vector<S, K> {
    type Output = Self;

    fn div(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] / rhs;
        }
        self
    }
}