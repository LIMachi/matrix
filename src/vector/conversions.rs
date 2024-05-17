use crate::matrix::Matrix;
use super::Vector;

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

impl <const S: usize, K: Copy + Default> From<Vector<S, K>> for Matrix<1, S, K> {
    fn from(value: Vector<S, K>) -> Self {
        let mut out = Matrix::<1, S, K>::default();
        for s in 0..S {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_from_array() {
        dbg!(Vector::from([0; 4]));
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
        let mat = Matrix::from(Vector::<4, f32>::default());
        println!("{}", mat);
        dbg!(mat);
    }

    #[test]
    fn test_matrix_to_vector() {
        dbg!(Vector::from(Matrix::<1, 4, f32>::default()));
    }
}