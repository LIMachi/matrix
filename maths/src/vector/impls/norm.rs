use crate::prelude::{Norm, Vector};
use crate::utils::babylonian_sqrt;

impl <const S: usize, K: Norm> Norm for Vector<S, K> {
    //manathan norm: addition of the length of each terms of the vector
    fn norm_1(&self) -> f32 {
        let mut acc = 0f32;
        for i in 0..S {
            acc += self[i].norm_1();
        }
        acc
    }

    //euclidian norm: direct length of the vector (air distance)
    fn norm(&self) -> f32 {
        let mut acc = 0f32;
        for i in 0..S {
            let t = self[i].norm();
            acc += t * t;
        }
        babylonian_sqrt(acc)
    }

    //uniform norm: maximum length of each term of the vector
    fn norm_inf(&self) -> f32 {
        let mut acc = 0f32;
        for i in 0..S {
            acc = acc.max(self[i].norm_inf());
        }
        acc
    }
}