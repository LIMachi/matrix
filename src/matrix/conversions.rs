use crate::vector::Vector;
use super::Matrix;

impl <const C: usize, const R: usize, K: Default + Copy> From<[[K; C]; R]> for Matrix<C, R, K> {
    fn from(value: [[K; C]; R]) -> Self {
        let mut t = Vector::default();
        for i in 0..R {
            t[i] = Vector::from(value[i]);
        }
        Self(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_from_array() {
        let mat = &Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        dbg!(mat);
        println!("{}", mat);
    }
}