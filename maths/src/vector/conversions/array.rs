use crate::prelude::{Vector, DynVec};

impl <const S: usize, K> From<[K; S]> for Vector<S, K> {
    fn from(value: [K; S]) -> Self {
        Self(value)
    }
}

impl <const S: usize, K: Copy> From<[K; S]> for DynVec<K> {
    fn from(value: [K; S]) -> Self {
        Self(value.iter().copied().collect())
    }
}

impl <const S: usize, K> From<Vector<S, K>> for [K; S] {
    fn from(value: Vector<S, K>) -> Self {
        value.0
    }
}

impl <const S: usize, K: Default + Copy> From<DynVec<K>> for [K; S] {
    fn from(value: DynVec<K>) -> Self {
        let mut out = [K::default(); S];
        for (i, v) in value.0.iter().enumerate() {
            if i >= S {
                break;
            }
            out[i] = *v;
        }
        out
    }
}