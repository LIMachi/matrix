use std::ops::{Add, Div, Mul, Neg};
use crate::matrix::Matrix;
use crate::utils::Unit;
use crate::vector::Vector;

//https://en.wikipedia.org/wiki/Rank_(linear_algebra)
//https://www.youtube.com/watch?v=uQhTuRlWMxw&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=8
//https://www.emathhelp.net/en/calculators/linear-algebra/rank-of-matrix-calculator/

//rank: number of independent columns
//so if the matrix is used for solving equations of the form:
//ax + by + cz = v1
//dx + ey + fz = v2
//gx + hy + iz = v3
//an independent columns means that if a is non null, then d and g are null, and so x is present only once in the operations
//this also means that there is only 1 solution to x in that case, and the rank can be interpreted as the number of solvable variables x/y/z with a singular solution
//to calculate the independent columns, we do a transformation to row echelon form, and in this form row and columns independence is equivalent (see visual result using transpose of row echelon form)
//and just add the number of rows that are non-null

impl <const C: usize, const R: usize, K: Default + PartialEq + Copy + Div<Output = K> + Neg<Output = K> + Mul<Output = K> + Add<Output = K> + Unit> Matrix<C, R, K> {
    pub fn rank(&self) -> usize {
        let rref = self.row_echelon();
        let cmp = Vector::<C, K>::default();
        let mut count = 0;
        for r in 0..R {
            if rref[r] != cmp {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat3;
    use super::*;

    #[test]
    fn test_0_identity() {
        assert_eq!(dbg!(Mat3::identity().rank()), 3);
    }

    #[test]
    fn test_1_mat43() {
        assert_eq!(dbg!(Matrix::from([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]).rank()), 2);
    }

    #[test]
    fn test_2_mat34() {
        assert_eq!(dbg!(Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]).rank()), 3);
    }
}