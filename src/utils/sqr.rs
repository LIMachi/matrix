use crate::complex::Complex;
use crate::utils::{Sqrf32};

impl Sqrf32 for f32 { fn sqrf32(self) -> f32 { self * self } }
impl Sqrf32 for f64 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for i8 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for i16 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for i32 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for i64 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for i128 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for u8 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for u16 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for u32 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for u64 { fn sqrf32(self) -> f32 { (self * self) as f32 } }
impl Sqrf32 for u128 { fn sqrf32(self) -> f32 { (self * self) as f32 } }

impl Sqrf32 for Complex {
    fn sqrf32(self) -> f32 {
        self.r * self.r - self.i * self.i
    }
}