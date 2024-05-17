use std::fmt::{Debug, Display, Formatter};
use super::Matrix;
use crate::utils::Unit;
use crate::vector::Vector;

impl <const C: usize, const R: usize, K> Matrix<C, R, K> {

    pub fn shape() -> (usize, usize) { (C, R) }

    pub fn rows(&self) -> &Vector<R, Vector<C, K>> {
        &self.0
    }

    pub fn rows_mut(&mut self) -> &mut Vector<R, Vector<C, K>> {
        &mut self.0
    }

    pub fn row(&self, index: usize) -> &Vector<C, K> {
        assert!(index < R, "Invalid row index {index} for matrice of size {C} (columns) * {R} (rows)");
        &self.0[index]
    }

    pub fn row_mut(&mut self, index: usize) -> &mut Vector<C, K> {
        assert!(index < R, "Invalid row index {index} for matrice of size {C} (columns) * {R} (rows)");
        &mut self.0[index]
    }
}

impl <const M: usize, K: Default + Copy + Unit> Matrix<M, M, K> {
    pub fn identity() -> Self {
        let mut out = Self::default();
        let one = K::unit();
        for i in 0..M {
            out[(i, i)] = one;
        }
        out
    }
}

impl <const C: usize, const R: usize, K: Default + Copy> Default for Matrix<C, R, K> {
    fn default() -> Self {
        Self(Vector::default())
    }
}

impl <const C: usize, const R: usize, K: Debug> Debug for Matrix<C, R, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl <const C: usize, const R: usize, K: Display> Display for Matrix<C, R, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Matrix: [\n")?;
        for r in 0..R - 1 {
            f.write_str("\t")?;
            for c in 0..C - 1 {
                f.write_fmt(format_args!("{}, ", self.0[r][c]))?;
            }
            f.write_fmt(format_args!("{},\n", self.0[r][C - 1]))?;
        }
        f.write_str("\t")?;
        for c in 0..C - 1 {
            f.write_fmt(format_args!("{}, ", self.0[R - 1][c]))?;
        }
        f.write_fmt(format_args!("{}\n]\n", self.0[R - 1][C - 1]))
    }
}

impl <const C: usize, const R: usize, K: Clone> Clone for Matrix<C, R, K> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl <const C: usize, const R: usize, K: Copy> Copy for Matrix<C, R, K> {}

#[cfg(test)]
mod tests {
    use crate::matrix::Mat3;
    use super::*;

    #[test]
    fn test_display() {
        let mat = Matrix::<2, 4, f32>::default();
        dbg!(&mat);
        println!("{}", mat);
    }

    #[test]
    fn test_unit() {
        let mat = Mat3::identity();
        dbg!(&mat);
        println!("{}", mat);
    }
}