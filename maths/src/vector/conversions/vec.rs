use crate::prelude::{Vector, DynVec};

impl <const S: usize, K: Default + Copy> From<Vec<K>> for Vector<S, K> {
    fn from(value: Vec<K>) -> Self {
        let mut out = Self::default();
        for i in 0..S.min(value.len()) {
            out.0[i] = value[i];
        }
        out
    }
}

impl <const S: usize, K: Default + Copy> From<Vector<S, K>> for Vec<K> {
    fn from(value: Vector<S, K>) -> Self {
        Vec::from(value.0)
    }
}

impl <K> From<Vec<K>> for DynVec<K> {
    fn from(value: Vec<K>) -> Self {
        Self(value)
    }
}

impl <K> From<DynVec<K>> for Vec<K> {
    fn from(value: DynVec<K>) -> Self {
        value.0
    }
}