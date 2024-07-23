use std::ops::{Add, Div, Mul};
use crate::exercises::ex04::Ex04Norm;
use crate::result;
use crate::utils::ex;
use crate::vector::Vector;
use crate::exercises::ex03::Ex03dot;

pub trait Ex05angleCos<Rhs = Self> {
    type Output;

    fn angle_cos(&self, rhs: Rhs) -> Self::Output;
}

impl <const S: usize, K: Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K> + Ex04Norm + Default + From<f32>> Ex05angleCos<&Self> for Vector<S, K> {
    type Output = K;
    
    ///I decided to make this function a method of Vector
    ///also, I made the type of return value K (since the angle might not be real with vectors of complex numbers)
    fn angle_cos(&self, v: &Self) -> K {
        self.dot(v) / K::from(self.norm() * v.norm())
    }
}

impl <const S: usize, K: Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K> + Ex04Norm + Default + From<f32>> Ex05angleCos for Vector<S, K> {
    type Output = K;
    fn angle_cos(&self, rhs: Self) -> Self::Output { self.angle_cos(&rhs) }
}

pub fn ex05() {
    ex(5, "Cosine");
    result!(
        Vector::from([1., 0.]).angle_cos(Vector::from([1., 0.])),
        Vector::from([1., 0.]).angle_cos(Vector::from([0., 1.])),
        Vector::from([-1., 1.]).angle_cos(Vector::from([1., -1.])),
        Vector::from([2., 1.]).angle_cos(Vector::from([4., 2.])),
        Vector::from([1., 2., 3.]).angle_cos(Vector::from([4., 5., 6.])),
    );
}

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use crate::utils::assert_f32_eq;
    use super::*;

    #[test]
    fn test_0_collinear() {
        assert_f32_eq(dbg!({
            let u = Vector::from([1., 0.]);
            let v = Vector::from([1., 0.]);
            u.angle_cos(v)
        }), 1.);
    }

    #[test]
    fn test_1_perpendicular() {
        assert_f32_eq(dbg!({
            let u = Vector::from([1., 0.]);
            let v = Vector::from([0., 1.]);
            u.angle_cos(v)
        }), 0.);
    }

    #[test]
    fn test_2_opposites() {
        assert_f32_eq(dbg!({
            let u = Vector::from([-1., 1.]);
            let v = Vector::from([1., -1.]);
            u.angle_cos(v)
        }), -1.);
    }

    #[test]
    fn test_3_scaled() {
        assert_f32_eq(dbg!({
            let u = Vector::from([2., 1.]);
            let v = Vector::from([4., 2.]);
            u.angle_cos(v)
        }), 1.);
    }

    #[test]
    fn test_4_3d() {
        assert_f32_eq(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            u.angle_cos(v)
        }), 0.974631896316349);
    }

    #[test]
    fn test_5_complex() {
        dbg!({
            let u = Vector::from([Complex::from_i(2.), Complex::new(1., 1.)]);
            let v = Vector::from([Complex::new(1., 1.), Complex::from_i(2.)]);
            u.angle_cos(v)
        });
    }
}