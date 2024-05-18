use std::ops::{Mul, Sub};
use crate::vector::Vector;

impl <K: Mul<Output = K> + Sub<Output = K> + Copy> Vector<3, K> {
    ///once again, made this function a method of vector
    pub fn cross_product(&self, v: &Self) -> Self {
        Vector::from([
            self.0[1] * v.0[2] - self.0[2] * v.0[1],
            self.0[2] * v.0[0] - self.0[0] * v.0[2],
            self.0[0] * v.0[1] - self.0[1] * v.0[0],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_z_by_x() {
        assert_eq!(dbg!({
            let u = Vector::from([0., 0., 1.]);
            let v = Vector::from([1., 0., 0.]);
            u.cross_product(&v)
        }), Vector::from([0., 1., 0.]));
    }

    #[test]
    fn test_1_cross() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            u.cross_product(&v)
        }), Vector::from([-3., 6., -3.]));
    }

    #[test]
    fn test_2_negatives() {
        assert_eq!(dbg!({
            let u = Vector::from([4., 2., -3.]);
            let v = Vector::from([-2., -5., 16.]);
            u.cross_product(&v)
        }), Vector::from([17., -58., -16.]));
    }
}