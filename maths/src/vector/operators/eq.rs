use crate::prelude::Vector;

impl <const S: usize, K: PartialEq> PartialEq for Vector<S, K> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..S {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}