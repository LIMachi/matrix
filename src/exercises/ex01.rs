use std::ops::{Add, Mul};
use crate::vector::Vector;

pub fn linear_combination<const S: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>>(u: &[Vector<S, K>], coefs: &[K]) -> Vector<S, K> {
    assert_eq!(u.len(), coefs.len(), "Mismatched sizes of arrays: {} vectors for {} coefficients", u.len(), coefs.len());
    let mut acc = Vector::<S, K>::default();
    for i in 0..u.len() {
        acc = acc + u[i] * coefs[i];
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);

        println!("{}", linear_combination::<3, f32>(&[e1, e2, e3], &[10., -2., 0.5]));
        println!("{}", linear_combination::<3, f32>(&[v1, v2], &[10., -2.]));
    }
}