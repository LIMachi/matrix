use std::ops::{Index, IndexMut};

use crate::prelude::{Matrix, Vector, DynMat, DynVec};

impl <const C: usize, const R: usize, K> Index<usize> for Matrix<C, R, K> {
    type Output = Vector<C, K>;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < R, "Invalid row index {index} for matrix of size {C} (columns) * {R} (rows)");
        &self.0[index]
    }
}

impl <const C: usize, const R: usize, K> IndexMut<usize> for Matrix<C, R, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < R, "Invalid column index {index} for matrix of size {C} (columns) * {R} (rows)");
        &mut self.0[index]
    }
}

impl <K> Index<usize> for DynMat<K> {
    type Output = DynVec<K>;

    fn index(&self, index: usize) -> &Self::Output {
        let rows = self.0.0.len();
        assert!(index < rows, "Invalid row index {index} for matrix of size {} (columns) * {rows} (rows)", if rows > 0 { self.0[0].0.len() } else { 0 });
        &self.0[index]
    }
}

impl <K> IndexMut<usize> for DynMat<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let rows = self.0.0.len();
        assert!(index < rows, "Invalid row index {index} for matrix of size {} (columns) * {rows} (rows)", if rows > 0 { self.0[0].0.len() } else { 0 });
        &mut self.0[index]
    }
}

impl <K> Index<(usize, usize)> for DynMat<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let rows = self.0.0.len();
        let columns = if rows > 0 { self.0[0].0.len() } else { 0 };
        assert!(index.0 < columns, "Invalid column index {} for matrix of size {columns} (columns) * {rows} (rows)", index.0);
        assert!(index.1 < rows, "Invalid row index {} for matrix of size {columns} (columns) * {rows} (rows)", index.1);
        &self.0[index.1][index.0]
    }
}

impl <K> IndexMut<(usize, usize)> for DynMat<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let rows = self.0.0.len();
        let columns = if rows > 0 { self.0[0].0.len() } else { 0 };
        assert!(index.0 < columns, "Invalid column index {} for matrix of size {columns} (columns) * {rows} (rows)", index.0);
        assert!(index.1 < rows, "Invalid row index {} for matrix of size {columns} (columns) * {rows} (rows)", index.1);
        &mut self.0[index.1][index.0]
    }
}

impl <const C: usize, const R: usize, K> Index<(usize, usize)> for Matrix<C, R, K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < C, "Invalid column index {} for matrix of size {C} (columns) * {R} (rows)", index.0);
        assert!(index.1 < R, "Invalid row index {} for matrix of size {C} (columns) * {R} (rows)", index.1);
        &self.0[index.1][index.0]
    }
}

impl <const C: usize, const R: usize, K> IndexMut<(usize, usize)> for Matrix<C, R, K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < C, "Invalid column index {} for matrix of size {C} (columns) * {R} (rows)", index.0);
        assert!(index.1 < R, "Invalid row index {} for matrix of size {C} (columns) * {R} (rows)", index.1);
        &mut self.0[index.1][index.0]
    }
}