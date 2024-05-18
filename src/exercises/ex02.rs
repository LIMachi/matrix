use std::ops::{Add, Mul};

pub trait Lerp {
    ///I decided to make this function a method of any type that implement the correct operations
    fn lerp(&self, v: Self, t: f32) -> Self;
}

impl <T: Add<Output = T> + Mul<f32, Output = T> + Copy> Lerp for T {
    fn lerp(&self, v: Self, t: f32) -> Self {
        *self * (1. - t) + v * t
    }
}

#[cfg(test)]
mod tests {
    use super::Lerp;
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_0_lerp_f32() {
        assert_eq!(dbg!(0f32.lerp(1., 0.)), 0.);
        assert_eq!(dbg!(0f32.lerp(1., 1.)), 1.);
        assert_eq!(dbg!(0f32.lerp(1., 0.5)), 0.5);
        assert_eq!(dbg!(21f32.lerp(42., 0.3)), 27.3);
    }

    #[test]
    fn test_1_lerp_vector() {
        assert_eq!(dbg!(Vector::from([2., 1.]).lerp(Vector::from([4., 2.]), 0.3)), Vector::from([2.6, 1.3]));
    }

    #[test]
    fn test_2_lerp_matrix() {
        assert_eq!(dbg!(Matrix::from([[2., 1.], [3., 4.]]).lerp(Matrix::from([[20., 10.], [30., 40.]]), 0.5)), Matrix::from([[11., 5.5], [16.5, 22.]]));
    }

    #[test]
    fn test_3_lerp_overflow() {
        assert_eq!(dbg!(Vector::from([1., 1.]).lerp(Vector::from([2., 2.]), 2.)), Vector::from([3., 3.]));
    }
}