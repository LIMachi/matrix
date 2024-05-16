use crate::vector::Vector;

mod conversions;
mod utilities;
mod operators;

pub struct Matrix<const C: usize, const R: usize, K>(pub(crate) Vector<R, Vector<C, K>>);