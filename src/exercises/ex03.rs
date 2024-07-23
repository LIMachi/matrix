use std::ops::{Add, Mul};
use crate::result;
use crate::utils::ex;
use crate::vector::Vector;

pub trait Ex03dot<Rhs = Self> {
    type Output;
    fn dot(&self, rhs: Rhs) -> Self::Output;
}

impl <const S: usize, K: Add<Output = K> + Mul<Output = K> + Copy + Default> Ex03dot<&Self> for Vector<S, K> {
    type Output = K;

    fn dot(&self, other: &Self) -> Self::Output {
        let mut acc = K::default();
        for i in 0..S {
            acc = acc + self[i] * other[i];
        }
        acc
    }
}

impl <const S: usize, K: Add<Output = K> + Mul<Output = K> + Copy + Default> Ex03dot for Vector<S, K> {
    type Output = K;
    fn dot(&self, rhs: Self) -> Self::Output { self.dot(&rhs) }
}

pub fn ex03() {
    ex(3, "Dot product");
    result!(
        Vector::from([0., 0.]).dot(Vector::from([1., 1.])),
        Vector::from([1., 1.]).dot(Vector::from([1., 1.])),
        Vector::from([-1., 6.]).dot(Vector::from([3., 2.]))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_null_dot() {
        assert_eq!(dbg!({
            let u = Vector::from([0., 0.]);
            let v = Vector::from([1., 1.]);
            u.dot(v)
        }), 0.);
    }

    #[test]
    fn test_1_dot() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 1.]);
            let v = Vector::from([1., 1.]);
            u.dot(v)
        }), 2.);
    }

    #[test]
    fn test_2_negative_dot() {
        assert_eq!(dbg!({
            let u = Vector::from([-1., 6.]);
            let v = Vector::from([3., 2.]);
            u.dot(v)
        }), 9.);
    }
}