use std::ops::{Index, IndexMut};

use crate::prelude::{Vector, DynVec};

impl <const S: usize, K> Index<usize> for Vector<S, K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < S, "Invalid index {index} for vector of size {S}");
        &self.0[index]
    }
}

impl <const S: usize, K> IndexMut<usize> for Vector<S, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < S, "Invalid index {index} for vector of size {S}");
        &mut self.0[index]
    }
}

impl <K> Index<usize> for DynVec<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.0.len(), "Invalid index {index} for vector of size {}", self.0.len());
        &self.0[index]
    }
}

impl <K> IndexMut<usize> for DynVec<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.0.len(), "Invalid index {index} for vector of size {}", self.0.len());
        &mut self.0[index]
    }
}

impl <const S: usize, K> Index<char> for Vector<S, K> {
    type Output = K;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' if S > 0 => &self.0[0],
            'y' if S > 1 => &self.0[1],
            'z' if S > 2 => &self.0[2],
            'w' if S > 3 => &self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {S}"),
        }
    }
}

impl <const S: usize, K> IndexMut<char> for Vector<S, K> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' if S > 0 => &mut self.0[0],
            'y' if S > 1 => &mut self.0[1],
            'z' if S > 2 => &mut self.0[2],
            'w' if S > 3 => &mut self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {S}"),
        }
    }
}

impl <K> Index<char> for DynVec<K> {
    type Output = K;

    fn index(&self, index: char) -> &Self::Output {
        let l = self.0.len();
        match index {
            'x' if l > 0 => &self.0[0],
            'y' if l > 1 => &self.0[1],
            'z' if l > 2 => &self.0[2],
            'w' if l > 3 => &self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {l}"),
        }
    }
}

impl <K> IndexMut<char> for DynVec<K> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        let l = self.0.len();
        match index {
            'x' if l > 0 => &mut self.0[0],
            'y' if l > 1 => &mut self.0[1],
            'z' if l > 2 => &mut self.0[2],
            'w' if l > 3 => &mut self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {l}"),
        }
    }
}