use crate::matrix::{Mat3, Matrix};
use crate::result;
use crate::utils::ex;

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

pub fn ex09() {
    ex(9, "Transpose");
    result!(
        Mat3::identity(),
        Mat3::identity().transpose(),
        Matrix::from([[4., 4.], [2., 8.]]).transpose()
    );
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat3;
    use super::*;

    #[test]
    fn test_transpose() {
        assert_eq!(dbg!(Mat3::identity().transpose()), Mat3::identity());
        assert_eq!(dbg!(Matrix::from([[4., 2.], [6., 9.]]).transpose()), Matrix::from([[4., 6.], [2., 9.]]));
    }
}