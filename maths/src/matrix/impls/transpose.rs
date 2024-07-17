use crate::prelude::Matrix;

impl <const C: usize, const R: usize, K: Copy + Default> Matrix<C, R, K> {
    pub fn transpose(&self) -> Matrix<R, C, K> {
        let mut out = Matrix::default();
        for c in 0..C {
            for r in 0..R {
                out[(r, c)] = self[(c, r)];
            }
        }
        out
    }
}