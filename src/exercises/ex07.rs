use std::fmt::Debug;
use std::ops::{Add, Mul};
use crate::matrix::Matrix;
use crate::vector::Vector;

impl <const C: usize, const R: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K> + Debug> Matrix<C, R, K> {
    pub fn mul_vec(&self, vec: &Vector<R, K>) -> Vector<C, K> {
        let mut out = Vector::<C, K>::default();
        for i in 0..R {
            for n in 0..C {
                out[i] = out[i] + vec[n] * self[(n, i)];
            }
        }
        out
    }

    pub fn mul_mat<const P: usize>(&self, mat: &Matrix<R, P, K>) -> Matrix<C, P, K> {
        let mut out = Matrix::<C, P, K>::default();
        for p in 0..P {
            for c in 0..C {
                for r in 0..R {
                    out[(c, p)] = out[(c, p)] + self[(c, r)] * mat[(r, p)];
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_unit_mat_x_vec() {
        dbg!({
            let u = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            let v = Vector::<2, f64>::from([4., 2.]);
            u.mul_vec(&v)
        });
    }

    #[test]
    fn test_1_mat_x_vec() {
        dbg!({
            let u = Matrix::from([
                [2., 0.],
                [0., 2.],
            ]);
            let v = Vector::<2, f64>::from([4., 2.]);
            u.mul_vec(&v)
        });
    }

    #[test]
    fn test_2_neg_mat_x_vec() {
        dbg!({
            let u = Matrix::from([
                [2., -2.],
                [-2., 2.],
            ]);
            let v = Vector::<2, f64>::from([4., 2.]);
            u.mul_vec(&v)
        });
    }

    #[test]
    fn test_3_unit_mat_x_unit_mat() {
        dbg!({
            let u = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            let v = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            u.mul_mat(&v)
        });
    }

    #[test]
    fn test_4_unit_mat_x_mat() {
        dbg!({
            let u = Matrix::from([
                [1., 0.],
                [0., 1.],
            ]);
            let v = Matrix::from([
                [2., 1.],
                [4., 2.],
            ]);
            u.mul_mat(&v)
        });
    }

    #[test]
    fn test_5_mat_x_mat() {
        dbg!({
            let u = Matrix::from([
                [3., -5.],
                [6., 8.],
            ]);
            let v = Matrix::from([
                [2., 1.],
                [4., 2.],
            ]);
            u.mul_mat(&v)
        });
    }
}