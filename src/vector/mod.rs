pub mod utilities;
pub mod conversions;
pub mod operators;

pub struct Vector<const S: usize, K>(pub(crate) [K; S]);

pub type Vec3 = Vector<3, f32>;
pub type Vec4 = Vector<4, f32>;