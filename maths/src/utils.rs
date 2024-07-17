use std::ops::{Add, Mul};

use crate::complex::Complex;
use crate::quaternion::Quaternion;

///helper trait used to get the value of 1 for random types (and also flag types that do not have a value of 1 available)
///https://github.com/rust-lang/rfcs/pull/3490
///still waiting for const trait functions :)
pub trait Unit {
    fn unit() -> Self;
}

pub trait Norm {
    fn norm_1(&self) -> f32;
    fn norm(&self) -> f32;
    fn norm_inf(&self) -> f32;
}

///for any type that can be represented as a single real, the length/norm is always abs of self
impl <T: Copy + Into<f32>> Norm for T where f32: From<T> {
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

impl Unit for f32 { fn unit() -> Self { 1. } }
impl Unit for f64 { fn unit() -> Self { 1. } }
impl Unit for i8 { fn unit() -> Self { 1 } }
impl Unit for i16 { fn unit() -> Self { 1 } }
impl Unit for i32 { fn unit() -> Self { 1 } }
impl Unit for i64 { fn unit() -> Self { 1 } }
impl Unit for i128 { fn unit() -> Self { 1 } }
impl Unit for u8 { fn unit() -> Self { 1 } }
impl Unit for u16 { fn unit() -> Self { 1 } }
impl Unit for u32 { fn unit() -> Self { 1 } }
impl Unit for u64 { fn unit() -> Self { 1 } }
impl Unit for u128 { fn unit() -> Self { 1 } }

impl Unit for Complex {
    fn unit() -> Self {
        Self {
            r: 1.,
            i: 0.
        }
    }
}

impl Unit for Quaternion {
    fn unit() -> Self {
        Self {
            r: 1.,
            i: 0.,
            j: 0.,
            k: 0.
        }
    }
}

//value=positive normalized number (not nan, inf, denormalized or <0) -> 1/sqrt(value)
//derived from formula seen there: https://www.youtube.com/watch?v=p8u_k2LIZyo
pub fn quake_rsqrt(value: f32) -> f32 {
    //in rust, direct conversion of ref to pointer of different type is not allowed, so I do a temporary cast to an intermediary pointer before casting it to the final pointer
    //in terms of safety: there is none, if the input value is not normal (ex: infinity, NaN or other degenerate forms) this will most likely result in NaN/Infinity
    //if value is < 0, it will result in an underflow (in release mode, it will usually result in -Inf)
    let mut y = unsafe { *(&(0x5f3759df - (*(&value as *const f32 as *const u32) >> 1)) as *const u32 as *const f32) };
    y = y * (1.5 - value * 0.5 * y * y);
    y * (1.5 - value * 0.5 * y * y)
}

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

pub trait Lerp {
    ///I decided to make this function a method of any type that implement the correct operations
    fn lerp(&self, v: Self, t: f32) -> Self;
}

impl <T: Add<Output = T> + Mul<f32, Output = T> + Copy> Lerp for T {
    fn lerp(&self, v: Self, t: f32) -> Self {
        *self * (1. - t) + v * t
    }
}

#[test]
fn test_quake_rsqrt() {
    dbg!(quake_rsqrt(1.)); //tend to 1
    dbg!(quake_rsqrt(0.)); //tend to positive infinity (divide by zero)
    dbg!(quake_rsqrt(4.)); //tend to 0.5
    dbg!(quake_rsqrt(2.));
    dbg!({
        let t = 1. / quake_rsqrt(2.);
        t * t
    }); //tend to 2.
    dbg!(quake_rsqrt(f32::NAN)); //NaN
    dbg!(quake_rsqrt(f32::INFINITY)); //inf
    #[cfg(not(debug_assertions))]
    { //results in underflow error in debug mode (and yes, you can run tests in release mode)
        dbg!(quake_rsqrt(-1.)); //-inf
        dbg!(quake_rsqrt(f32::NEG_INFINITY)); //-inf
    }
}