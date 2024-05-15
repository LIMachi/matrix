use std::ops::{Add, Mul};

pub fn lerp<V: Add<Output = V> + Mul<f32, Output = V>>(u: V, v: V, t: f32) -> V {
    u * (1. - t) + v * t
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::vector::Vector;
    use super::lerp;

    #[test]
    fn test_0_lerp_f32() {
        dbg!(lerp(0., 1., 0.));
        dbg!(lerp(0., 1., 1.));
        dbg!(lerp(0., 1., 0.5));
    }

    #[test]
    fn test_1_lerp_vector() {
        dbg!(lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
    }

    #[test]
    fn test_2_lerp_matrix() {
        dbg!(lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20., 10.], [30., 40.]]), 0.5));
    }

    #[test]
    fn test_3_lerp_overflow() {
        dbg!(lerp(Vector::from([1., 1.]), Vector::from([2., 2.]), 2.));
    }
}