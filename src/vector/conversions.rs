use crate::matrix::Matrix;
use super::Vector;

impl <K> From<K> for Vector<1, K> {
    fn from(value: K) -> Self {
        Self([value; 1])
    }
}

impl <const S: usize, K> From<[K; S]> for Vector<S, K> {
    fn from(value: [K; S]) -> Self {
        Self(value)
    }
}

impl <const S: usize, K> From<Vector<S, K>> for [K; S] {
    fn from(value: Vector<S, K>) -> Self {
        value.0
    }
}

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

impl <const S: usize, K> From<Vector<S, K>> for Matrix<1, S, K> {
    fn from(value: Vector<S, K>) -> Self {
        Matrix::<1, S, K>(Vector::from(value))
    }
}

impl <const _D: usize, const S: usize, K: Copy> From<Matrix<_D, S, K>> for Vector<S, K> {
    fn from(value: Matrix<_D, S, K>) -> Self {
        *value.column(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_vector() {
        dbg!(Vector::from(0));
    }

    #[test]
    fn test_vector_from_array() {
        dbg!(Vector::<4, i32>::from([0; 4]));
    }

    #[test]
    fn test_vector_to_array() {
        dbg!(<[f32; 4]>::from(Vector::<4, f32>::default()));
    }

    #[test]
    fn test_vector_from_vec() {
        dbg!(Vector::<4, i32>::from(vec![0; 4]));
    }

    #[test]
    fn test_vector_to_vec() {
        dbg!(Vec::from(Vector::<4, f32>::default()));
    }

    #[test]
    fn test_vector_to_matrix() {
        dbg!(Matrix::<1, 4, f32>::from(Vector::<4, f32>::default()));
    }

    #[test]
    fn test_matrix_to_vector() {
        dbg!(Vector::<4, f32>::from(Matrix::<1, 4, f32>::default()));
    }
}