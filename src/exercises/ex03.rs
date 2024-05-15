use std::ops::{Add, Mul};
use crate::vector::Vector;

impl <const S: usize, K: Add<Output = K> + Mul<Output = K> + Copy + Default> Vector<S, K> {
    pub fn dot(&self, other: Self) -> K {
        let mut acc = K::default();
        for i in 0..S {
            acc = acc + self[i] * other[i];
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_dot() {
        dbg!({
            let u = Vector::<2, f32>::from([0., 0.]);
            let v = Vector::from([1., 1.]);
            u.dot(v)
        });
    }

    #[test]
    fn test_dot() {
        dbg!({
            let u = Vector::<2, f32>::from([1., 1.]);
            let v = Vector::from([1., 1.]);
            u.dot(v)
        });
    }

    #[test]
    fn test_negative_dot() {
        dbg!({
            let u = Vector::<2, f32>::from([-1., 6.]);
            let v = Vector::from([3., 2.]);
            u.dot(v)
        });
    }
}