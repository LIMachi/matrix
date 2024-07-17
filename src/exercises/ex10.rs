use maths::matrix::{Mat3, Matrix};
use utils::{result, show, ex};

///note: the examples given in the pdf expect the reduced form of row echelon, not the basic one
///otherwise the result of [[1,2],[3,4]] would be [[1,2],[0,-2]], not [[1,0],[0,1]]

pub fn ex10() {
    ex(10, "Row-echelon form");
    result!(
        Mat3::identity().row_echelon(),
        Matrix::from([[1., 2.], [3., 4.]]).row_echelon(),
        Matrix::from([[1., 2.], [2., 4.]]).row_echelon(),
    );
    let r0;
    let r1;
    let r2;
    show!(
        r0 = [8., 5., -2., 4., 28.],
        r1 = [4., 2.5, 20., 4., -4.],
        r2 = [8., 5., 1., 4., 17.]
    );
    result!(Matrix::from([r0, r1, r2]).row_echelon());
}

#[cfg(test)]
mod tests {
    use maths::prelude::Mat3;
    use super::*;

    #[test]
    fn test_0_unit() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
            u.row_echelon()
        }), Mat3::identity());
    }

    #[test]
    fn test_1_no_zeroes() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., 2.], [3., 4.]]);
            u.row_echelon()
        }), Matrix::<2, 2, f32>::identity());
    }

    #[test]
    fn test_2_big_mat() {
        assert_eq!(dbg!({
            let u = Matrix::<5, 3, f32>::from([[8., 5., -2., 4., 28.], [4., 2.5, 20., 4., -4.], [8., 5., 1., 4., 17.]]);
            u.row_echelon()
        }), Matrix::from([[1., 0.625, 0., 0., -12.166668], [0., 0., 1., 0., -3.6666667], [0., 0., 0., 1., 29.5 ]]));
    }
}