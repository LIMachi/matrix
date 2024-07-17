use std::ops::Mul;
use utils::{result, show, ex};
use maths::prelude::{Vector, Matrix};

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
    use maths::prelude::{Vector, Matrix};

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