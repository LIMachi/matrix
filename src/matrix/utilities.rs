use std::fmt::{Debug, Display, Formatter};
use crate::matrix::Matrix;
use crate::vector::Vector;

impl <const C: usize, const R: usize, K> Matrix<C, R, K> {

    pub fn shape() -> (usize, usize) { (C, R) }

    pub fn columns(&self) -> &Vector<C, Vector<R, K>> {
        &self.0
    }

    pub fn columns_mut(&mut self) -> &mut Vector<C, Vector<R, K>> {
        &mut self.0
    }

    pub fn column(&self, index: usize) -> &Vector<R, K> {
        assert!(index < C, "Invalid column index {index} for matrice of size {R} (rows) * {C} (columns)");
        &self.0[index]
    }

    pub fn column_mut(&mut self, index: usize) -> &mut Vector<R, K> {
        assert!(index < C, "Invalid column index {index} for matrice of size {R} (rows) * {C} (columns)");
        &mut self.0[index]
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
                f.write_fmt(format_args!("{}, ", self.0[c][r]))?;
            }
            f.write_fmt(format_args!("{},\n", self.0[C - 1][r]))?;
        }
        f.write_str("\t")?;
        for c in 0..C - 1 {
            f.write_fmt(format_args!("{}, ", self.0[c][R - 1]))?;
        }
        f.write_fmt(format_args!("{}\n]\n", self.0[C - 1][R - 1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let mat = Matrix::<2, 4, f32>::default();
        dbg!(&mat);
        println!("{}", mat);
    }
}