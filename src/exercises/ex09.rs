use maths::matrix::{Mat3, Matrix};
use utils::{result, ex};

pub fn ex09() {
    ex(9, "Transpose");
    result!(
        Mat3::identity(),
        Mat3::identity().transpose(),
        Matrix::from([[4., 4.], [2., 8.]]).transpose()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        assert_eq!(dbg!(Mat3::identity().transpose()), Mat3::identity());
        assert_eq!(dbg!(Matrix::from([[4., 2.], [6., 9.]]).transpose()), Matrix::from([[4., 6.], [2., 9.]]));
    }
}