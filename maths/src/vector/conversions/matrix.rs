use crate::prelude::{Vector, DynVec, Matrix, DynMat};

impl <const S: usize, K: Copy + Default> From<Vector<S, K>> for Matrix<1, S, K> {
    fn from(value: Vector<S, K>) -> Self {
        let mut out = Matrix::<1, S, K>::default();
        for s in 0..S {
            out[(0, s)] = value[s];
        }
        out
    }
}

impl <K: Copy + Default> From<DynVec<K>> for DynMat<K> {
    fn from(value: DynVec<K>) -> Self {
        let mut out = DynMat::with_size(1, value.0.len());
        for s in 0..value.0.len() {
            out[(0, s)] = value[s];
        }
        out
    }
}

impl <const _D: usize, const S: usize, K: Copy + Default> From<Matrix<_D, S, K>> for Vector<S, K> {
    fn from(value: Matrix<_D, S, K>) -> Self {
        let mut out = Self::default();
        for s in 0..S {
            out[s] = value[(0, s)];
        }
        out
    }
}
