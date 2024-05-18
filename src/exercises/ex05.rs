use std::ops::{Add, Div, Mul};
use crate::exercises::ex04::Norm;
use crate::vector::Vector;

impl <const S: usize, K: Copy + Add<Output = K> + Mul<Output = K> + Div<Output = K> + Norm + Default + From<f32>> Vector<S, K> {
    ///I decided to make this function a method of Vector
    ///also, I made the type of return value K (since the angle might not be real with vectors of complex numbers)
    pub fn angle_cos(&self, v: &Self) -> K {
        self.dot(v) / K::from(self.norm() * v.norm())
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_f32_eq;
    use crate::complex::Complex;
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