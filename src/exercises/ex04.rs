use crate::complex::Complex;
use crate::{result, show};
use crate::utils::ex;
use crate::vector::Vector;

///note: the subject requires self to be mutable, but by definition, the norm of a vector does not require transformation, so I removed the superfluous mut
///I allowed myself to use abs for f32 (as it is basically: `if a < 0. { -a } else { a }`)
///I also had to generalize the norm to work on non vector to work with complex numbers

///since sqrt is not allowed, I had to redo it
///https://blogs.sas.com/content/iml/2016/05/16/babylonian-square-roots.html

#[cfg(not(feature = "std-sqrt"))]
pub fn babylonian_sqrt(val: f32) -> f32 {
    if val <= 0. {
        0. //handles negative numbers as 0 (should result in an error or complex number to be exact)
    } else if val == 1. {
        1.
    } else {
        let mut mean = (val + 1.) / 2.;
        for _ in 0..8 { //use 8 iterations by default (since f32 has a maximal exponent of 7 bits, I use one more to be extra cautious)
            let estimate = val / mean;
            mean = (mean + estimate) / 2.;
            let t = mean * mean;
            if t + f32::EPSILON > val && t - f32::EPSILON < val {
                break; //if we are close enough (in f32 error range), stop the guessing early
            }
        }
        mean
    }
}

#[cfg(feature = "std-sqrt")]
#[inline]
pub fn babylonian_sqrt(val: f32) -> f32 {
    val.sqrt()
}

pub trait Ex04Norm {
    fn norm_1(&self) -> f32;
    fn norm(&self) -> f32;
    fn norm_inf(&self) -> f32;
}

///for any type that can be represented as a single real, the length/norm is always abs of self
impl <T: Copy + Into<f32>> Ex04Norm for T where f32: From<T> {
    fn norm_1(&self) -> f32 {
        f32::from(*self).abs()
    }

    fn norm(&self) -> f32 {
        f32::from(*self).abs()
    }

    fn norm_inf(&self) -> f32 {
        f32::from(*self).abs()
    }
}

impl Ex04Norm for Complex {
    fn norm_1(&self) -> f32 {
        self.r.abs() + self.i.abs()
    }

    fn norm(&self) -> f32 {
        babylonian_sqrt(self.r * self.r + self.i * self.i)
    }

    fn norm_inf(&self) -> f32 {
        self.r.abs().max(self.i.abs())
    }
}

impl <const S: usize, K: Ex04Norm> Ex04Norm for Vector<S, K> {
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

impl <const S: usize> Vector<S, f32> {
    pub fn normalized(&self) -> Self {
        *self * (1. / self.norm())
    }
}

pub fn ex04() {
    ex(4, "Norm");
    let mut u;
    show!(u = Vector::from([0., 0., 0.]));
    result!(u.norm_1(), u.norm(), u.norm_inf());
    show!(u = Vector::from([1., 2., 3.]));
    result!(u.norm_1(), u.norm(), u.norm_inf());
    let u;
    show!(u = Vector::from([-1., -2.]));
    result!(u.norm_1(), u.norm(), u.norm_inf());
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