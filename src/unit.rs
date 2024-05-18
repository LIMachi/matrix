use crate::complex::Complex;
use crate::quaternion::Quaternion;

///helper trait used to get the value of 1 for random types (and also flag types that do not have a value of 1 available)
///https://github.com/rust-lang/rfcs/pull/3490
///still waiting for const trait functions :)
pub trait Unit {
    fn unit() -> Self;
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