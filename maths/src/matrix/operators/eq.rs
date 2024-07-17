use crate::prelude::Matrix;

impl <const C: usize, const R: usize, K: PartialEq> PartialEq for Matrix<C, R, K> {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..R {
            for c in 0..C {
                if self[(c, r)] != other[(c, r)] {
                    return false;
                }
            }
        }
        true
    }
}