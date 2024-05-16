use std::ops::{Add, Mul, Sub};
use crate::matrix::Matrix;
use crate::vector::Vector;

pub trait Ex00add {
    fn add(&mut self, other: &Self);
}

pub trait Ex00sub {
    fn sub(&mut self, other: &Self);
}

pub trait Ex00scl<K: Mul<Output = K> + Copy> {
    fn scl(&mut self, scalar: K);
}

impl <const S: usize, K: Add<Output = K> + Copy> Ex00add for Vector<S, K> {
    fn add(&mut self, other: &Self) {
        for i in 0..S {
            self[i] = self[i] + other[i];
        }
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Ex00sub for Vector<S, K> {
    fn sub(&mut self, other: &Self) {
        for i in 0..S {
            self[i] = self[i] - other[i];
        }
    }
}

impl <const S: usize, K: Mul<Output = K> + Copy> Ex00scl<K> for Vector<S, K> {
    fn scl(&mut self, scalar: K) {
        for i in 0..S {
            self[i] = self[i] * scalar;
        }
    }
}

impl <const R: usize, const C: usize, K: Add<Output = K> + Copy> Ex00add for Matrix<R, C, K> {
    fn add(&mut self, other: &Self) {
        for i in 0..C {
            self.row_mut(i).add(other.row(i));
        }
    }
}

impl <const R: usize, const C: usize, K: Sub<Output = K> + Copy> Ex00sub for Matrix<R, C, K> {
    fn sub(&mut self, other: &Self) {
        for i in 0..C {
            self.row_mut(i).sub(other.row(i));
        }
    }
}

impl <const R: usize, const C: usize, K: Mul<Output = K> + Copy> Ex00scl<K> for Matrix<R, C, K> {
    fn scl(&mut self, scalar: K) {
        for i in 0..C {
            self.row_mut(i).scl(scalar);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use crate::matrix::Matrix;
    use super::{Ex00add, Ex00sub, Ex00scl};

    #[test]
    fn test_0_add_vectors() {
        assert_eq!(dbg!({
            let mut u = Vector::from([2., 3.]);
            let v = Vector::from([5., 7.]);
            u.add(&v);
            u
        }), Vector::from([7., 10.]));
    }

    #[test]
    fn test_1_sub_vectors() {
        assert_eq!(dbg!({
            let mut u = Vector::from([2., 3.]);
            let v = Vector::from([5., 7.]);
            u.sub(&v);
            u
        }), Vector::from([-3., -4.]));
    }

    #[test]
    fn test_2_scl_vector() {
        assert_eq!(dbg!({
            let mut u = Vector::from([2., 3.]);
            u.scl(2.);
            u
        }), Vector::from([4., 6.]));
    }

    #[test]
    fn test_3_add_matrices() {
        assert_eq!(dbg!({
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u.add(&v);
            u
        }), Matrix::from([[8., 6.], [1., 6.]]));
    }

    #[test]
    fn test_4_sub_matrices() {
        assert_eq!(dbg!({
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u.sub(&v);
            u
        }), Matrix::from([[-6., -2.], [5., 2.]]));
    }

    #[test]
    fn test_5_scl_matrix() {
        assert_eq!(dbg!({
            let mut u = Matrix::from([[1., 2.], [3., 4.]]);
            u.scl(2.);
            u
        }), Matrix::from([[2., 4.], [6., 8.]]));
    }
}