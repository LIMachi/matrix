use crate::complex::Complex;
use crate::utils::Unit;

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