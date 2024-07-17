use crate::vector::{DynVec, Vector};

pub mod conversions;
pub mod utilities;
pub mod operators;
pub mod impls;

pub struct Matrix<const C: usize, const R: usize, K>(pub(crate) Vector<R, Vector<C, K>>);
#[derive(Default)]
pub struct DynMat<K>(pub(crate) DynVec<DynVec<K>>);

pub type Mat3 = Matrix<3, 3, f32>;
pub type Mat4 = Matrix<4, 4, f32>;