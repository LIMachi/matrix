use utils::{result, ex};
use maths::prelude::Vector;

pub fn ex05() {
    ex(5, "Cosine");
    result!(
        Vector::from([1., 0.]).angle_cos(&Vector::from([1., 0.])),
        Vector::from([1., 0.]).angle_cos(&Vector::from([0., 1.])),
        Vector::from([-1., 1.]).angle_cos(&Vector::from([1., -1.])),
        Vector::from([2., 1.]).angle_cos(&Vector::from([4., 2.])),
        Vector::from([1., 2., 3.]).angle_cos(&Vector::from([4., 5., 6.])),
    );
}

#[cfg(test)]
mod tests {
    use maths::complex::Complex;
    use crate::assert_f32_eq;
    use super::*;

    #[test]
    fn test_0_collinear() {
        assert_f32_eq(dbg!({
            let u = Vector::from([1., 0.]);
            let v = Vector::from([1., 0.]);
            u.angle_cos(&v)
        }), 1.);
    }

    #[test]
    fn test_1_perpendicular() {
        assert_f32_eq(dbg!({
            let u = Vector::from([1., 0.]);
            let v = Vector::from([0., 1.]);
            u.angle_cos(&v)
        }), 0.);
    }

    #[test]
    fn test_2_opposites() {
        assert_f32_eq(dbg!({
            let u = Vector::from([-1., 1.]);
            let v = Vector::from([1., -1.]);
            u.angle_cos(&v)
        }), -1.);
    }

    #[test]
    fn test_3_scaled() {
        assert_f32_eq(dbg!({
            let u = Vector::from([2., 1.]);
            let v = Vector::from([4., 2.]);
            u.angle_cos(&v)
        }), 1.);
    }

    #[test]
    fn test_4_3d() {
        assert_f32_eq(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            u.angle_cos(&v)
        }), 0.974631896316349);
    }

    #[test]
    fn test_5_complex() {
        dbg!({
            let u = Vector::from([Complex::from_i(2.), Complex::new(1., 1.)]);
            let v = Vector::from([Complex::new(1., 1.), Complex::from_i(2.)]);
            u.angle_cos(&v)
        });
    }
}