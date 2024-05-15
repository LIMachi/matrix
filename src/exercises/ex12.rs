use crate::matrix::Matrix;

pub enum MatrixInverseError {}

//https://en.wikipedia.org/wiki/Invertible_matrix

impl <const M: usize, K> Matrix<M, M, K> {
    pub fn inverse(&self) -> Result<Matrix<M, M, K>, MatrixInverseError> {
        todo!()
    }
}