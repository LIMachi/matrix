use std::ops::{Mul, Sub, Add};

use crate::prelude::Matrix;

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