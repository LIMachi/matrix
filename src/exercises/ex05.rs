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
        assert_eq!(dbg!({
            let u = Vector::from([1., 0.]);
            let v = Vector::from([1., 0.]);
            angle_cos(&u, &v)
        }), 1.);
    }

    #[test]
    fn test_1_perpendicular() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 0.]);
            let v = Vector::from([0., 1.]);
            angle_cos(&u, &v)
        }), 0.);
    }

    #[test]
    fn test_2_opposites() {
        assert_eq!(dbg!({
            let u = Vector::from([-1., 1.]);
            let v = Vector::from([1., -1.]);
            angle_cos(&u, &v)
        }), -1.); //assert fails due to the sqrt of 2 having an error of 10^-7 in f32
    }

    #[test]
    fn test_3_demonstrate_sqrt_of_2() {
        let v;
        dbg!({
            v = 2f32.sqrt();
            v
        });
        dbg!(v * v);
        let v64;
        dbg!({
            v64 = 2f64.sqrt();
            v64
        });
        dbg!(v64 * v64);
        assert_eq!(v64 * v64, 2.); //the error on sqrt of 2 is even significant enough on f64 to result in an error
    }

    #[test]
    fn test_4_scaled() {
        assert_eq!(dbg!({
            let u = Vector::from([2., 1.]);
            let v = Vector::from([4., 2.]);
            angle_cos(&u, &v)
        }), 1.);
    }

    #[test]
    fn test_5_3d() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            angle_cos(&u, &v)
        }), 0.9746319);
    }
}