use std::ops::{Add, Div, Mul, Neg};
use crate::matrix::Matrix;
use crate::unit::Unit;

///note: the examples given in the pdf expect the reduced form of row echelon, not the basic one
///otherwise the result of [[1,2],[3,4]] would be [[1,2],[0,-2]], not [[1,0],[0,1]]

impl <const C: usize, const R: usize, K: Default + PartialEq + Copy + Div<Output = K> + Neg<Output = K> + Mul<Output = K> + Add<Output = K> + Unit> Matrix<C, R, K> {
    pub fn row_echelon(&self) -> Matrix<C, R, K> {
        let zero = K::default();
        let one = K::unit();
        let mut out = *self;
        let mut pivot = 0;
        for row in 0..R {
            if out.0[row][pivot] == zero {
                //bubbling case -> reorder rows in echelon form and update pivot position if needed
                while pivot < C {
                    let mut r = row + 1;
                    while r < R && out.0[r][pivot] == zero {
                        r += 1;
                    }
                    if r < R {
                        //found bubbling pivot
                        let t = out.0[r];
                        out.0[r] = out.0[row];
                        out.0[row] = t;
                        break;
                    } else {
                        //only zeroes where found, move pivot right
                        pivot += 1;
                        if pivot < C && out.0[row][pivot] != zero { //next pivot is non zero, no need to check for bubbling
                            break;
                        }
                    }
                }
            }
            if pivot < C {
                //reduce (make sure the pivot is 1)
                if out.0[row][pivot] != one {
                    out.0[row] = out.0[row] / out.0[row][pivot];
                }
                //solve other rows using this row
                for r in 0..R {
                    if r != row {
                        let s = -out.0[r][pivot];
                        if s != zero {
                            out.0[r] = out.0[r] + out.0[row] * s;
                        }
                    }
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat3;
    use super::*;

    #[test]
    fn test_0_unit() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            u.row_echelon()
        }), Mat3::identity());
    }

    #[test]
    fn test_1_no_zeroes() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., 2.], [3., 4.]]);
            u.row_echelon()
        }), Matrix::<2, 2, f32>::identity());
    }

    #[test]
    fn test_2_big_mat() {
        assert_eq!(dbg!({
            let u = Matrix::<5, 3, f32>::from([[8., 5., -2., 4., 28.], [4., 2.5, 20., 4., -4.], [8., 5., 1., 4., 17.]]);
            u.row_echelon()
        }), Matrix::from([[1., 0.625, 0., 0., -12.166668], [0., 0., 1., 0., -3.6666667], [0., 0., 0., 1., 29.5 ]]));
    }
}