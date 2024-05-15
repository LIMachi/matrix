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
            self.column_mut(i).add(other.column(i));
        }
    }
}

impl <const R: usize, const C: usize, K: Sub<Output = K> + Copy> Ex00sub for Matrix<R, C, K> {
    fn sub(&mut self, other: &Self) {
        for i in 0..C {
            self.column_mut(i).sub(other.column(i));
        }
    }
}

impl <const R: usize, const C: usize, K: Mul<Output = K> + Copy> Ex00scl<K> for Matrix<R, C, K> {
    fn scl(&mut self, scalar: K) {
        for i in 0..C {
            self.column_mut(i).scl(scalar);
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
        let mut u = Vector::<2, f32>::from([2., 3.]);
        let v = Vector::<2, f32>::from([5., 7.]);
        u.add(&v);
        println!("{}", u);
    }

    #[test]
    fn test_1_sub_vectors() {
        let mut u = Vector::<2, f32>::from([2., 3.]);
        let v = Vector::<2, f32>::from([5., 7.]);
        u.sub(&v);
        println!("{}", u);
    }

    #[test]
    fn test_2_scl_vector() {
        let mut u = Vector::<2, f32>::from([2., 3.]);
        u.scl(2.);
        println!("{}", u);
    }

    #[test]
    fn test_3_add_matrices() {
        let mut u = Matrix::<2, 2, f32>::from([[1., 2.], [3., 4.]]);
        let v = Matrix::<2, 2, f32>::from([[7., 4.], [-2., 2.]]);
        u.add(&v);
        dbg!(&u);
        println!("{}", u);
    }

    #[test]
    fn test_4_sub_matrices() {
        let mut u = Matrix::<2, 2, f32>::from([[1., 2.], [3., 4.]]);
        let v = Matrix::<2, 2, f32>::from([[7., 4.], [-2., 2.]]);
        u.sub(&v);
        dbg!(&u);
        println!("{}", u);
    }

    #[test]
    fn test_5_scl_matrix() {
        let mut u = Matrix::<2, 2, f32>::from([[1., 2.], [3., 4.]]);
        u.scl(2.);
        dbg!(&u);
        println!("{}", u);
    }
}