use std::ops::{Add, Div, Mul, Neg};
use crate::matrix::Matrix;
use crate::utils::Unit;

#[derive(Debug)]
pub struct MatrixInverseError;

//https://en.wikipedia.org/wiki/Invertible_matrix
//https://www.youtube.com/watch?v=uQhTuRlWMxw&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=8
//https://www.emathhelp.net/en/calculators/linear-algebra/inverse-of-matrix-calculator/

//I will be using the gauss-jordan elimination (as it is basically a row-echelon solver with a few constraints)
//each operation on the left matrix must be mirrored on the right matrix
//if the left matrix ends up in identity, then the right matrix is the inverse of the input otherwise this matrix is not invertible

impl <const M: usize, K: Default + Copy + Unit + PartialEq + Div<Output = K> + Neg<Output = K> + Add<Output = K> + Mul<Output = K>> Matrix<M, M, K> {
    pub fn inverse(&self) -> Result<Matrix<M, M, K>, MatrixInverseError> {
        let zero = K::default();
        let one = K::unit();
        let identity = Self::identity();

        if self == &identity {
            return Ok(identity);
        }

        let mut left = *self;
        let mut right = identity;

        let mut pivot = 0;

        for row in 0..M {
            if left.0[row][pivot] == zero {
                while pivot < M {
                    let mut r = row + 1;
                    while r < M && left.0[r][pivot] == zero {
                        r += 1;
                    }
                    if r < M {
                        let t = left.0[r];
                        left.0[r] = left.0[row];
                        left.0[row] = t;
                        let t = right.0[r];
                        right.0[r] = right.0[row];
                        right.0[row] = t;
                        break;
                    } else {
                        pivot += 1;
                        if pivot < M && left.0[row][pivot] != zero {
                            break;
                        }
                    }
                }
            }
            if pivot < M {
                let rp = left.0[row][pivot];
                if rp != one {
                    left.0[row] = left.0[row] / rp;
                    right.0[row] = right.0[row] / rp;
                }
                for r in 0..M {
                    if r != row {
                        let s = -left.0[r][pivot];
                        if s != zero {
                            left.0[r] = left.0[r] + left.0[row] * s;
                            right.0[r] = right.0[r] + right.0[row] * s;
                        }
                    }
                }
            }
        }
        if left == identity {
            Ok(right)
        } else {
            Err(MatrixInverseError)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat3;
    use super::*;

    #[test]
    fn test_0_identity() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [1., 0., 0.],
                [0., 1., 0.],
                [0., 0., 1.],
            ]);
            u.inverse().unwrap()
        }), Mat3::identity());
    }

    #[test]
    fn test_1_half() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [2., 0., 0.],
                [0., 2., 0.],
                [0., 0., 2.],
            ]);
            u.inverse().unwrap()
        }), Mat3::identity() * 0.5);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(dbg!({
            let u = Mat3::from([
                [8., 5., -2.],
                [4., 7., 20.],
                [7., 6., 1.],
            ]);
            u.inverse().unwrap()
        }), Matrix::from([[0.64942527, 0.09770115, -0.6551724], [-0.7816092, -0.12643678, 0.9655172], [0.14367816, 0.07471265, -0.20689656]]));
    }

    #[test]
    fn test_3_invalid() {
        dbg!({
            let u = Matrix::from([
                [1., 1.],
                [1., 1.]
            ]);
            u.inverse()
        }).ok();
    }
}