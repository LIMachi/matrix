use crate::{prelude::Norm, utils::babylonian_sqrt};

pub mod utilities;
pub mod conversions;
pub mod operators;

pub struct Complex {
    pub(crate) r: f32,
    pub(crate) i: f32,
}

impl Norm for Complex {
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