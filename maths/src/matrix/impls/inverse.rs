use std::ops::{Div, Neg, Add, Mul};

use crate::prelude::{Unit, Matrix};

#[derive(Debug)]
pub struct MatrixInverseError;

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
