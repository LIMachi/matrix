use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use crate::vector::Vector;

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


impl <const S: usize, K> Index<char> for Vector<S, K> {
    type Output = K;

    fn index(&self, index: char) -> &Self::Output {
        match index {
            'x' | 'r' if S > 0 => &self.0[0],
            'y' | 'i' if S > 1 => &self.0[1],
            'z' | 'j' if S > 2 => &self.0[2],
            'w' | 'k' if S > 3 => &self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {S}"),
        }
    }
}

impl <const S: usize, K> IndexMut<char> for Vector<S, K> {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        match index {
            'x' | 'r' if S > 0 => &mut self.0[0],
            'y' | 'i' if S > 1 => &mut self.0[1],
            'z' | 'j' if S > 2 => &mut self.0[2],
            'w' | 'k' if S > 3 => &mut self.0[3],
            _ => panic!("Invalid label access {index} for vector of size {S}"),
        }
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> Add for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> Add<K> for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs;
        }
        self
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Sub for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Sub<K> for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs;
        }
        self
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> Mul for Vector<S, K> {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] * rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> Mul<K> for Vector<S, K> {
    type Output = Self;

    fn mul(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> Div for Vector<S, K> {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] / rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Div<Output = K> + Copy> Div<K> for Vector<S, K> {
    type Output = Self;

    fn div(mut self, rhs: K) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] / rhs;
        }
        self
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_accessor() {
        dbg!(Vector::<4, f32>::from([1., 2., 3., 4.])[2]);
    }

    #[test]
    fn test_label_accessor() {
        dbg!(Vector::<4, f32>::from([1., 2., 3., 4.])['y']);
    }

    #[test]
    fn test_operator_add() {
        dbg!(Vector::<2, i32>::from([1, 2]) + Vector::<2, i32>::from([3, 2]));
        dbg!(Vector::<2, i32>::from([1, 2]) + 2);
    }

    #[test]
    fn test_operator_sub() {
        dbg!(Vector::<2, i32>::from([1, 2]) - Vector::<2, i32>::from([3, 2]));
        dbg!(Vector::<2, i32>::from([1, 2]) - 2);
    }

    #[test]
    fn test_operator_mul() {
        dbg!(Vector::<2, i32>::from([1, 2]) * Vector::<2, i32>::from([3, 2]));
        dbg!(Vector::<2, i32>::from([1, 2]) * 2);
    }

    #[test]
    fn test_operator_div() {
        dbg!(Vector::<2, i32>::from([1, 2]) / Vector::<2, i32>::from([3, 2]));
        dbg!(Vector::<2, i32>::from([1, 2]) / 2);
    }
}