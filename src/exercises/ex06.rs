use std::ops::{Mul, Sub};
use crate::vector::Vector;

pub fn cross_product<K: Mul<Output = K> + Sub<Output = K> + Copy>(u: &Vector<3, K>, v: &Vector<3, K>) -> Vector<3, K> {
    Vector::from([
        u.0[1] * v.0[2] - u.0[2] * v.0[1],
        u.0[2] * v.0[0] - u.0[0] * v.0[2],
        u.0[0] * v.0[1] - u.0[1] * v.0[0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_z_by_x() {
        dbg!({
            let u = Vector::from([0., 0., 1.]);
            let v = Vector::from([1., 0., 0.]);
            cross_product(&u, &v)
        });
    }

    #[test]
    fn test_1_cross() {
        dbg!({
            let u = Vector::from([1., 2., 3.]);
            let v = Vector::from([4., 5., 6.]);
            cross_product(&u, &v)
        });
    }

    #[test]
    fn test_2_negatives() {
        dbg!({
            let u = Vector::from([4., 2., -3.]);
            let v = Vector::from([-2., -5., 16.]);
            cross_product(&u, &v)
        });
    }
}