use std::ops::Add;
use crate::matrix::Matrix;
use crate::result;
use crate::utils::ex;

impl <const M: usize, K: Add<Output = K> + Default + Copy> Matrix<M, M, K> {
    pub fn trace(&self) -> K {
        let mut trace = K::default();
        for i in 0..M {
            trace = trace + self[(i, i)];
        }
        trace
    }
}

pub fn ex08() {
    ex(8, "Trace");
    result!(
        Matrix::from([[1., 0.], [0., 1.]]).trace(),
        Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]).trace(),
        Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]).trace(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_trace_unit() {
        assert_eq!(dbg!(Matrix::from([[1., 0.], [0., 1.]]).trace()), 2.);
    }

    #[test]
    fn test_1_trace() {
        assert_eq!(dbg!(Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]).trace()), 9.);
    }

    #[test]
    fn test_2_neg_trace() {
        assert_eq!(dbg!(Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]).trace()), -21.);
    }
}