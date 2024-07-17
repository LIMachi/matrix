use std::ops::{Div, Neg, Mul, Add};

use crate::prelude::{Matrix, Vector, Unit};

impl <const C: usize, const R: usize, K: Default + PartialEq + Copy + Div<Output = K> + Neg<Output = K> + Mul<Output = K> + Add<Output = K> + Unit> Matrix<C, R, K> {
    pub fn rank(&self) -> usize {
        let rref = self.row_echelon();
        let cmp = Vector::<C, K>::default();
        let mut count = 0;
        for r in 0..R {
            if rref[r] != cmp {
                count += 1;
            }
        }
        count
    }
}