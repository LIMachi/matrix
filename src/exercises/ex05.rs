use std::ops::{Add, Mul, Sub};
use crate::utils::{Absf32, Sqrf32};
use crate::vector::Vector;

pub fn angle_cos<const S: usize, K: Into<f32> + Copy + Add<Output = K> + Default + Sqrf32 + Sub<Output = K> + Absf32 + Mul<Output = K>>(u: &Vector<S, K>, v: &Vector<S, K>) -> f32 {
    u.dot(v).into() / (u.norm() * v.norm())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_collinear() {
        dbg!({
            let u = Vector::<2, f32>::from([1., 0.]);
            let v = Vector::<2, f32>::from([1., 0.]);
            angle_cos(&u, &v)
        });
    }

    #[test]
    fn test_1_perpendicular() {
        dbg!({
            let u = Vector::<2, f32>::from([1., 0.]);
            let v = Vector::<2, f32>::from([0., 1.]);
            angle_cos(&u, &v)
        });
    }

    #[test]
    fn test_2_opposites() {
        dbg!({
            let u = Vector::<2, f32>::from([-1., 1.]);
            let v = Vector::<2, f32>::from([1., -1.]);
            angle_cos(&u, &v)
        });
    }

    #[test]
    fn test_3_scaled() {
        dbg!({
            let u = Vector::<2, f32>::from([2., 1.]);
            let v = Vector::<2, f32>::from([4., 2.]);
            angle_cos(&u, &v)
        });
    }

    #[test]
    fn test_4_3d() {
        dbg!({
            let u = Vector::<3, f32>::from([1., 2., 3.]);
            let v = Vector::<3, f32>::from([4., 5., 6.]);
            angle_cos(&u, &v)
        });
    }
}