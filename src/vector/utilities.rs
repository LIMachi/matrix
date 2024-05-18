use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Index, Mul};
use crate::unit::Unit;
use super::Vector;

impl <const S: usize, K: Default + Copy> Default for Vector<S, K> {
    fn default() -> Self {
        Self([K::default(); S])
    }
}

impl <const S: usize, K: Debug> Debug for Vector<S, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl <const S: usize, K: Display> Display for Vector<S, K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Vector: [")?;
        for i in 0..S - 1 {
            f.write_fmt(format_args!("{}, ", self.0[i]))?;
        }
        f.write_fmt(format_args!("{}]\n", self.0[S - 1]))
    }
}

impl <const S: usize, K> Vector<S, K> {
    pub fn size() -> usize { S }
}

impl <const S: usize, K: Clone> Clone for Vector<S, K>{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl <const S: usize, K: Copy> Copy for Vector<S, K>{}

impl <const S: usize, K: Copy + Default> Vector<S, K> {
    pub fn swizzle<const L: usize, I: Copy>(&self, indexes: [I; L]) -> Vector<L, K> where Vector<S, K>: Index<I, Output = K> {
        let mut out = Vector::<L, K>::default();
        for l in 0..L {
            out[l] = self[indexes[l]];
        }
        out
    }
}

impl <const S: usize, K: Copy + Default + Unit + PartialEq + Add<Output = K> + Mul<Output = K> + Div<Output = K>> Vector<S, K> {
    pub fn normalize(&self) -> Self {
        let l = self.dot(&self);
        if l != K::unit() && l != K::default() {
            *self / l
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_default() {
        dbg!(Vector::<4, Vector<4, f32>>::default());
    }

    #[test]
    fn test_display() {
        println!("{}", Vector::<2, i32>::default());
    }

    #[test]
    fn test_swizzle() {
        dbg!(Vector::from([0., 1., 2., 3.]).swizzle([3, 2, 3, 0]));
    }
}