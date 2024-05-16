use std::ops::{Add, Mul, Sub};
use crate::matrix::Matrix;

//https://en.wikipedia.org/wiki/Determinant

//TODO: watch 3 blue 1 brown since the subject ask for it

impl <K: Copy> Matrix<1, 1, K> {
    pub fn determinant(&self) -> K {
        self.0[0][0]
    }
}

impl <K: Copy + Mul<Output = K> + Sub<Output = K>> Matrix<2, 2, K> {
    pub fn determinant(&self) -> K {
        self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]
    }
}

impl <K: Copy + Mul<Output = K> + Sub<Output = K> + Add<Output = K>> Matrix<3, 3, K> {
    pub fn determinant(&self) -> K {
        self.0[0][0] * self.0[1][1] * self.0[2][2]
            + self.0[0][1] * self.0[1][2] * self.0[2][0]
            + self.0[0][2] * self.0[1][0] * self.0[2][1]
            - self.0[0][2] * self.0[1][1] * self.0[2][0]
            - self.0[0][1] * self.0[1][0] * self.0[2][2]
            - self.0[0][0] * self.0[1][2] * self.0[2][1]
    }
}

//call me crazy, but I think I can do it using 0 iterations :P
impl <K: Copy + Mul<Output = K> + Sub<Output = K> + Add<Output = K>> Matrix<4, 4, K> {
    pub fn determinant(&self) -> K {
            self.0[0][0] * self.0[1][1] * self.0[2][2] * self.0[3][3]
                + self.0[0][0] * self.0[2][1] * self.0[3][2] * self.0[1][3]
                + self.0[0][0] * self.0[3][1] * self.0[1][2] * self.0[2][3]
                - self.0[0][0] * self.0[3][1] * self.0[2][2] * self.0[1][3]
                - self.0[0][0] * self.0[2][1] * self.0[1][2] * self.0[3][3]
                - self.0[0][0] * self.0[1][1] * self.0[3][2] * self.0[2][3]
                - self.0[1][0] * self.0[0][1] * self.0[2][2] * self.0[3][3]
                - self.0[2][0] * self.0[0][1] * self.0[3][2] * self.0[1][3]
                - self.0[3][0] * self.0[0][1] * self.0[1][2] * self.0[2][3]
                + self.0[3][0] * self.0[0][1] * self.0[2][2] * self.0[1][3]
                + self.0[2][0] * self.0[0][1] * self.0[1][2] * self.0[3][3]
                + self.0[1][0] * self.0[0][1] * self.0[3][2] * self.0[2][3]
                + self.0[1][0] * self.0[2][1] * self.0[0][2] * self.0[3][3]
                + self.0[2][0] * self.0[3][1] * self.0[0][2] * self.0[1][3]
                + self.0[3][0] * self.0[1][1] * self.0[0][2] * self.0[2][3]
                - self.0[3][0] * self.0[2][1] * self.0[0][2] * self.0[1][3]
                - self.0[2][0] * self.0[1][1] * self.0[0][2] * self.0[3][3]
                - self.0[1][0] * self.0[3][1] * self.0[0][2] * self.0[2][3]
                - self.0[1][0] * self.0[2][1] * self.0[3][2] * self.0[0][3]
                - self.0[2][0] * self.0[3][1] * self.0[1][2] * self.0[0][3]
                - self.0[3][0] * self.0[1][1] * self.0[2][2] * self.0[0][3]
                + self.0[3][0] * self.0[2][1] * self.0[1][2] * self.0[0][3]
                + self.0[2][0] * self.0[1][1] * self.0[3][2] * self.0[0][3]
                + self.0[1][0] * self.0[3][1] * self.0[2][2] * self.0[0][3]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_null_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., -1.], [-1., 1.]]);
            u.determinant()
        }), 0.);
    }

    #[test]
    fn test_1_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
            u.determinant()
        }), 8.);
    }

    #[test]
    fn test_2_neg_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
            u.determinant()
        }), -174.);
    }

    #[test]
    fn test_3_big_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[8., 5., -2., 4.], [4., 2.5, 20., 4.], [8., 5., 1., 4.], [28., -4., 17., 1.]]);
            u.determinant()
        }), 1032.);
    }
}