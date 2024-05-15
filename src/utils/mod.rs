mod abs;
mod sqr;

///note: the subject of ex04 clearly states that those function should work with vector of complex numbers and the result should always be real
///this implies the use of complex absolute formula |a+bi| -> sqrt(a² + b²) so |i| -> 1, because of this, I expect K to implement abs (and will implement it myself for absolute numbers for the bonuses)
///https://www.varsitytutors.com/hotmath/hotmath_help/topics/absolute-value-complex-number
pub trait Absf32 {
    fn absf32(self) -> f32;
}

pub trait Sqrf32 {
    fn sqrf32(self) -> f32;
}

/// note: since most exercises do not allow sqrt, here is a brute force version that should be stable enough for demonstration purpose using the babylonian method
///https://blogs.sas.com/content/iml/2016/05/16/babylonian-square-roots.html
pub fn babylonian_sqrt(val: f32) -> f32 {
    if val <= 0. {
        0. //handles negative numbers as 0 (should result in an error or complex number to be exact)
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

#[test]
fn test_brute_force_sqrt() {
    dbg!(babylonian_sqrt(0.));
    dbg!(babylonian_sqrt(4.));
    dbg!(babylonian_sqrt(2.));
}