use crate::vector::Vector;

mod conversions;
mod utilities;
mod operators;

//column major order: array of columns (and each column is of size R aka rows)
///note: since the matrix is stored in column major order, the display and the debug produce different orders for the numbers
///ex: [[1, 2][3, 4]] -> Matrix: [1, 3, 2, 4]
pub struct Matrix<const C: usize, const R: usize, K>(pub(crate) Vector<C, Vector<R, K>>);