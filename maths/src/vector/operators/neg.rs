use std::ops::Neg;

use crate::prelude::Vector;

impl <const S: usize, K: Neg<Output=K> + Copy> Neg for Vector<S, K> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for i in 0..S {
            self[i] = -self[i];
        }
        self
    }
}
