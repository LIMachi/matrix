use std::ops::{Add, Mul};
use crate::matrix::Matrix;
use crate::result;
use crate::utils::ex;
use crate::vector::Vector;

///replaced mul_vec by the operator `mat * vec -> vec` and mul_mat by the operator `mat * mat -> mat`

impl <const C: usize, const R: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>> Mul<&Vector<R, K>> for Matrix<C, R, K> {
    type Output = Vector<C, K>;

    fn mul(self, rhs: &Vector<R, K>) -> Self::Output {
        let mut out = Vector::<C, K>::default();
        for i in 0..R {
            for n in 0..C {
                out[i] = out[i] + rhs[n] * self[(n, i)];
            }
        }
        out
    }
}

impl <const C: usize, const R: usize, const P: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>> Mul<&Matrix<P, C, K>> for Matrix<C, R, K> {
    type Output = Matrix<P, R, K>;

    fn mul(self, rhs: &Matrix<P, C, K>) -> Self::Output {
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

pub fn ex07() {
    ex(7, "Linear map, matrix multiplication");
    result!(
        Matrix::from([[1., 0.], [0., 1.]]) * Vector::from([4., 2.]),
        Matrix::from([[2., 0.], [0., 2.]]) * Vector::from([4., 2.]),
        Matrix::from([[2., -2.], [-2., 2.]]) * Vector::from([4., 2.]),
        Matrix::<2, 2, f64>::identity(),
        Matrix::<2, 2, f64>::identity() * Matrix::<2, 2, f64>::identity(),
        Matrix::<2, 2, f64>::identity() * Matrix::from([[2., 1.], [4., 2.]]),
        Matrix::from([[3., -5.], [6., 8.]]) * Matrix::from([[2., 1.], [4., 2.]])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_unit_mat_x_vec() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            let v = Vector::from([4., 2.]);
            u * v
        }), Vector::from([4., 2.]));
    }

    #[test]
    fn test_1_mat_x_vec() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [2., 0.],
                [0., 2.],
            ]);
            let v = Vector::from([4., 2.]);
            u * v
        }), Vector::from([8., 4.]));
    }

    #[test]
    fn test_2_neg_mat_x_vec() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [2., -2.],
                [-2., 2.],
            ]);
            let v = Vector::from([4., 2.]);
            u * v
        }), Vector::from([4., -4.]));
    }

    #[test]
    fn test_3_unit_mat_x_unit_mat() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            let v = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            u * v
        }), Matrix::<2, 2, f64>::identity());
    }

    #[test]
    fn test_4_unit_mat_x_mat() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            let v = Matrix::from([
                [2., 1.],
                [4., 2.],
            ]);
            u * v
        }), Matrix::from([[2., 1.], [4., 2.]]));
    }

    #[test]
    fn test_5_mat_x_mat() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [3., -5.],
                [6., 8.],
            ]);
            let v = Matrix::from([
                [2., 1.],
                [4., 2.],
            ]);
            u * v
        }), Matrix::from([[-14., -7.], [44., 22.]]));
    }

    #[test]
    fn test_6_mat_x_mat_with_size_change() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [1., 2., 3.],
                [4., 5., 6.],
            ]);
            let v = Matrix::from([
                [7., 8.],
                [9., 10.],
                [11., 12.]
            ]);
            u * v
        }), Matrix::from([[58., 64.], [139., 154.]]));
    }
}