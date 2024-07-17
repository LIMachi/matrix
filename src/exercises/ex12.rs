use maths::matrix::{Mat3, Matrix};
use utils::{result, show, ex};

//https://en.wikipedia.org/wiki/Invertible_matrix
//https://www.youtube.com/watch?v=uQhTuRlWMxw&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=8
//https://www.emathhelp.net/en/calculators/linear-algebra/inverse-of-matrix-calculator/

//I will be using the gauss-jordan elimination (as it is basically a row-echelon solver with a few constraints)
//each operation on the left matrix must be mirrored on the right matrix
//if the left matrix ends up in identity, then the right matrix is the inverse of the input otherwise this matrix is not invertible

pub fn ex12() {
    ex(12, "Inverse");
    result!(
        Mat3::identity().inverse().unwrap(),
        (Mat3::identity() * 2.).inverse().unwrap(),
    );
    let r;
    let r0;
    let r1;
    let r2;
    show!(
        r0 = [8., 5., -2.],
        r1 = [4., 7., 20.],
        r2 = [7., 6., 1.],
        r = Matrix::from([r0, r1, r2]).inverse().unwrap()
    );
    println!("{r:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_identity() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [1., 0., 0.],
                [0., 1., 0.],
                [0., 0., 1.],
            ]);
            u.inverse().unwrap()
        }), Mat3::identity());
    }

    #[test]
    fn test_1_half() {
        assert_eq!(dbg!({
            let u = Matrix::from([
                [2., 0., 0.],
                [0., 2., 0.],
                [0., 0., 2.],
            ]);
            u.inverse().unwrap()
        }), Mat3::identity() * 0.5);
    }

    #[test]
    fn test_2_hard() {
        assert_eq!(dbg!({
            let u = Mat3::from([
                [8., 5., -2.],
                [4., 7., 20.],
                [7., 6., 1.],
            ]);
            u.inverse().unwrap()
        }), Matrix::from([[0.64942527, 0.09770115, -0.6551724], [-0.7816092, -0.12643678, 0.9655172], [0.14367816, 0.07471265, -0.20689656]]));
    }

    #[test]
    fn test_3_invalid() {
        dbg!({
            let u = Matrix::from([
                [1., 1.],
                [1., 1.]
            ]);
            u.inverse()
        }).ok();
    }
}