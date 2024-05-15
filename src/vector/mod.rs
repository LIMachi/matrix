mod utilities;
mod conversions;
mod operators;

pub struct Vector<const S: usize, K>(pub(crate) [K; S]);