use std::ops::{Add, Sub};
use crate::utils::{Absf32, babylonian_sqrt, Sqrf32};
use crate::vector::Vector;

///note: the subject requires self to be mutable, but by definition, the norm of a vector does not require transformation, so I removed the superfluous mut
///since abs, sqr and sqrt are not allowed, I had to redo them
impl <const S: usize, K: Add<Output = K> + Sub<Output = K> + Copy + Default + Absf32 + Sqrf32> Vector<S, K> {
    //manathan norm: addition of the length of each terms of the vector
    pub fn norm_1(&self) -> f32 {
        let mut acc = 0f32;
        for i in 0..S {
            acc += self[i].absf32();
        }
        acc
    }

    //euclidian norm: direct length of the vector (air distance)
    pub fn norm(&self) -> f32 {
        let mut acc = 0f32;
        for i in 0..S {
            acc += self[i].sqrf32();
        }
        babylonian_sqrt(acc)
    }

    //uniform norm: maximum length of each term of the vector
    pub fn norm_inf(&self) -> f32 {
        let mut acc = 0f32;
        for i in 0..S {
            acc = acc.max(self[i].absf32());
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_null_vector() {
        assert_eq!(dbg!({
            let u = Vector::from([0., 0., 0.]);
            let n1 = u.norm_1();
            let n = u.norm();
            let ni = u.norm_inf();
            println!("{}, {}, {}", n1, n, ni);
            (n1, n, ni)
        }), (0., 0., 0.));
    }

    #[test]
    fn test_1_vector() {
        assert_eq!(dbg!({
            let u = Vector::from([1., 2., 3.]);
            let n1 = u.norm_1();
            let n = u.norm();
            let ni = u.norm_inf();
            println!("{}, {}, {}", n1, n, ni);
            (n1, n, ni)
        }), (6., 3.7416573, 3.));
    }

    #[test]
    fn test_2_negative_vector() {
        assert_eq!(dbg!({
            let u = Vector::from([-1., -2.]);
            let n1 = u.norm_1();
            let n = u.norm();
            let ni = u.norm_inf();
            println!("{}, {}, {}", n1, n, ni);
            (n1, n, ni)
        }), (3.,  2.2360679, 2.));
    }
}