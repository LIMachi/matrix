use std::ops::{Add, Mul};
use utils::{result, show, ex};
use maths::prelude::Vector;

///since newer versions of rust allow the use of const generic (generics that represent values instead of types),
///we can write methods that check at compile time that the parameters are of correct size
///(so I don't need to check at run time if u and coefficients are arrays of same size)
pub fn linear_combination<const S: usize, const A: usize, K: Default + Copy + Mul<Output = K> + Add<Output = K>>(u: &[Vector<S, K>; A], coefficients: &[K; A]) -> Vector<S, K> {
    let mut acc = Vector::<S, K>::default();
    for i in 0..u.len() {
        acc = acc + u[i] * coefficients[i];
    }
    acc
}

pub fn ex01() {
    ex(1, "Linear combination");

    let e1;
    let e2;
    let e3;
    let v1;
    let v2;

    show!(
        e1 = Vector::from([1., 0., 0.]),
        e2 = Vector::from([0., 1., 0.]),
        e3 = Vector::from([0., 0., 1.]),
        v1 = Vector::from([1., 2., 3.]),
        v2 = Vector::from([0., 10., -100.])
    );
    result!(
        linear_combination(&[e1, e2, e3], &[10., -2., 0.5]),
        linear_combination(&[v1, v2], &[10., -2.])
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_combination() {
        assert_eq!(dbg!({
            let e1 = Vector::from([1., 0., 0.]);
            let e2 = Vector::from([0., 1., 0.]);
            let e3 = Vector::from([0., 0., 1.]);
            let v1 = Vector::from([1., 2., 3.]);
            let v2 = Vector::from([0., 10., -100.]);

            let r1 = linear_combination(&[e1, e2, e3], &[10., -2., 0.5]);
            let r2 = linear_combination(&[v1, v2], &[10., -2.]);
            println!("{}", r1);
            println!("{}", r2);
            (r1, r2)
         }), (Vector::from([10., -2., 0.5]), Vector::from([10., 0., 230.])));
    }
}