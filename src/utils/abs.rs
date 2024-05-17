use crate::complex::Complex;
use crate::quaternion::Quaternion;
use super::{Absf32, babylonian_sqrt};

impl Absf32 for f32 { fn absf32(self) -> f32 { self.abs() } }
impl Absf32 for f64 { fn absf32(self) -> f32 { self.abs() as f32 } }
impl Absf32 for i8 { fn absf32(self) -> f32 { self.abs() as f32 } }
impl Absf32 for i16 { fn absf32(self) -> f32 { self.abs() as f32 } }
impl Absf32 for i32 { fn absf32(self) -> f32 { self.abs() as f32 } }
impl Absf32 for i64 { fn absf32(self) -> f32 { self.abs() as f32 } }
impl Absf32 for i128 { fn absf32(self) -> f32 { self.abs() as f32 } }

impl Absf32 for Complex {
    fn absf32(self) -> f32 {
        babylonian_sqrt(self.r * self.r + self.i * self.i)
    }
}

impl Absf32 for Quaternion {
    fn absf32(self) -> f32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_0() {
        dbg!(0f32.absf32());
    }

    #[test]
    fn test_abs_n2() {
        dbg!(-2f32.absf32());
    }

    #[test]
    fn test_abs_2() {
        dbg!(2f32.absf32());
    }
}