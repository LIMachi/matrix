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
        let u = Vector::<3, f32>::from([0., 0., 0.]);
        println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    }

    #[test]
    fn test_1_vector() {
        let u = Vector::<3, f32>::from([1., 2., 3.]);
        println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    }

    #[test]
    fn test_2_negative_vector() {
        let u = Vector::<2, f32>::from([-1., -2.]);
        println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    }
}