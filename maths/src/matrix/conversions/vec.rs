use crate::prelude::Matrix;

impl <const C: usize, const R: usize, K: Default + Copy> From<Matrix<C, R, K>> for Vec<K> {
    fn from(value: Matrix<C, R, K>) -> Self {
        let mut out = Vec::with_capacity(C * R);
        for c in 0..C {
            for r in 0..R {
                out.push(value[(c, r)]);
            }
        }
        out
    }
}