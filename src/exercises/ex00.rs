use std::ops::{Add, Mul, Sub};
use crate::matrix::Matrix;
use crate::{result, show};
use crate::utils::ex;
use crate::vector::Vector;

///instead of using the given prototypes (which use hold rust notation)
///I opted to implement add and sub as actual operator overloads
///and generalised scl method to any type that return itself when multiplied by a scalar

pub trait Ex00scl<K: Copy>: Mul<K, Output = Self> + Copy {
    fn scl(&mut self, scalar: K);
}

impl <K: Copy, T: Mul<K, Output = T> + Copy> Ex00scl<K> for T {
    fn scl(&mut self, scalar: K) {
        *self = *self * scalar
    }
}

impl <const S: usize, K: Add<Output = K> + Copy> Add<&Self> for Vector<S, K> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl <const S: usize, K: Sub<Output = K> + Copy> Sub<&Self> for Vector<S, K> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        for i in 0..S {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Add<Output = K> + Copy> Add<&Self> for Matrix<C, R, K> {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] + rhs[(c, r)];
            }
        }
        self
    }
}

impl <const C: usize, const R: usize, K: Sub<Output = K> + Copy> Sub<&Self> for Matrix<C, R, K> {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        for r in 0..R {
            for c in 0..C {
                self[(c, r)] = self[(c, r)] - rhs[(c, r)];
            }
        }
        self
    }
}

pub fn ex00() {
    ex(0, "Add, Subtract and Scale");

    let mut u;
    let v;

    println!("vector operations:");
    show!(
        u = Vector::from([2., 3.]),
        v = Vector::from([5., 7.])
    );
    result!(u + v, u - v, { u.scl(2.); u });
    println!();

    let mut u;
    let v;

    println!("matrix operations:");
    show!(
        u = Matrix::from([[1., 2.], [3., 4.]]),
        v = Matrix::from([[7., 4.], [-2., 2.]])
    );
    result!(u + v, u - v, { u.scl(2.); u });
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use crate::matrix::Matrix;
    use super::Ex00scl;

    #[test]
    fn test_0_add_vectors() {
        assert_eq!(dbg!({
            let u = Vector::from([2., 3.]);
            let v = Vector::from([5., 7.]);
            u + v
        }), Vector::from([7., 10.]));
    }

    #[test]
    fn test_1_sub_vectors() {
        assert_eq!(dbg!({
            let u = Vector::from([2., 3.]);
            let v = Vector::from([5., 7.]);
            u - v
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
            let u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u + v
        }), Matrix::from([[8., 6.], [1., 6.]]));
    }

    #[test]
    fn test_4_sub_matrices() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., 2.], [3., 4.]]);
            let v = Matrix::from([[7., 4.], [-2., 2.]]);
            u - v
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