use std::ops::{Add, Mul, Sub};
use crate::matrix::{Mat3, Matrix};
use crate::{result, show};
use crate::utils::ex;

//https://en.wikipedia.org/wiki/Determinant
//https://www.youtube.com/watch?v=Ip3X9LOh2dk&list=PL0-GT3co4r2y2YErbmuJw2L5tW4Ew2O5B&index=7

//what det represents geometrically ->

//deformation of the space (change of area/volume in case of scaling, or change of orientation/form in case of shearing)
//ex: det 6 means that all objects represented by points/vectors in the space will have a new area/volume 6 times bigger than before (does not indicate a change of shape though)
//det 1 means that all objects represented by points/vectors in the space might have their shape changed, but their area/volume will stay the same (ex: square 1*1 becomes a parallelogram 1*1)
//a negative determinant in 2d represents a flip in axis (that X and Y axis have been exchanged, but does not give us the exact rotation)
//in 3d, positive determinant means that all the axis are still in their original order: normal of the planes XY, YZ, XZ (XY -> aka cross product of the vectors [1, 0, 0] x [0, 1, 0]) are still pointing in the same relative direction
//but a negative determinant means that at least one of the normals is now inverted (ex: plane XY used to have a normal pointing towards Z, but now points towards -Z, while the other two planes still have positive normals)

//det 0 means that all vectors inside the space will now be collinear in 2d or on the same plane in 3d (and more generally, means that one dimension was lost)
//if the matrix represents the 2d space xy, but we rotate it in 3d space around the x or y axis by 90 degrees, then we end up with the y or x axis swapped with the z axis and we end up with a line from the 2d view xy

impl <K: Copy> Matrix<1, 1, K> {
    pub fn determinant(&self) -> K {
        self.0[0][0]
    }
}

impl <K: Copy + Mul<Output = K> + Sub<Output = K>> Matrix<2, 2, K> {
    pub fn determinant(&self) -> K {
        self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]
    }
}

impl <K: Copy + Mul<Output = K> + Sub<Output = K> + Add<Output = K>> Matrix<3, 3, K> {
    pub fn determinant(&self) -> K {
        self.0[0][0] * self.0[1][1] * self.0[2][2]
            + self.0[0][1] * self.0[1][2] * self.0[2][0]
            + self.0[0][2] * self.0[1][0] * self.0[2][1]
            - self.0[0][2] * self.0[1][1] * self.0[2][0]
            - self.0[0][1] * self.0[1][0] * self.0[2][2]
            - self.0[0][0] * self.0[1][2] * self.0[2][1]
    }
}

//call me crazy, but I think I can do it using 0 iterations :P
impl <K: Copy + Mul<Output = K> + Sub<Output = K> + Add<Output = K>> Matrix<4, 4, K> {
    pub fn determinant(&self) -> K {
            self.0[0][0] * self.0[1][1] * self.0[2][2] * self.0[3][3]
                + self.0[0][0] * self.0[2][1] * self.0[3][2] * self.0[1][3]
                + self.0[0][0] * self.0[3][1] * self.0[1][2] * self.0[2][3]
                - self.0[0][0] * self.0[3][1] * self.0[2][2] * self.0[1][3]
                - self.0[0][0] * self.0[2][1] * self.0[1][2] * self.0[3][3]
                - self.0[0][0] * self.0[1][1] * self.0[3][2] * self.0[2][3]
                - self.0[1][0] * self.0[0][1] * self.0[2][2] * self.0[3][3]
                - self.0[2][0] * self.0[0][1] * self.0[3][2] * self.0[1][3]
                - self.0[3][0] * self.0[0][1] * self.0[1][2] * self.0[2][3]
                + self.0[3][0] * self.0[0][1] * self.0[2][2] * self.0[1][3]
                + self.0[2][0] * self.0[0][1] * self.0[1][2] * self.0[3][3]
                + self.0[1][0] * self.0[0][1] * self.0[3][2] * self.0[2][3]
                + self.0[1][0] * self.0[2][1] * self.0[0][2] * self.0[3][3]
                + self.0[2][0] * self.0[3][1] * self.0[0][2] * self.0[1][3]
                + self.0[3][0] * self.0[1][1] * self.0[0][2] * self.0[2][3]
                - self.0[3][0] * self.0[2][1] * self.0[0][2] * self.0[1][3]
                - self.0[2][0] * self.0[1][1] * self.0[0][2] * self.0[3][3]
                - self.0[1][0] * self.0[3][1] * self.0[0][2] * self.0[2][3]
                - self.0[1][0] * self.0[2][1] * self.0[3][2] * self.0[0][3]
                - self.0[2][0] * self.0[3][1] * self.0[1][2] * self.0[0][3]
                - self.0[3][0] * self.0[1][1] * self.0[2][2] * self.0[0][3]
                + self.0[3][0] * self.0[2][1] * self.0[1][2] * self.0[0][3]
                + self.0[2][0] * self.0[1][1] * self.0[3][2] * self.0[0][3]
                + self.0[1][0] * self.0[3][1] * self.0[2][2] * self.0[0][3]
    }
}

pub fn ex11() {
    ex(11, "Determinant");
    result!(
        Matrix::from([[1., -1.], [-1., 1.]]).determinant(),
        Mat3::identity() * 2.,
        (Mat3::identity() * 2.).determinant(),
        Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]).determinant(),
    );
    let r0;
    let r1;
    let r2;
    let r3;
    show!(
        r0 = [8., 5., -2., 4.],
        r1 = [4., 2.5, 20., 4.],
        r2 = [8., 5., 1., 4.],
        r3 = [28., -4., 17., 1.],
    );
    result!(Matrix::from([r0, r1, r2, r3]).determinant());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_null_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[1., -1.], [-1., 1.]]);
            u.determinant()
        }), 0.);
    }

    #[test]
    fn test_1_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
            u.determinant()
        }), 8.);
    }

    #[test]
    fn test_2_neg_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
            u.determinant()
        }), -174.);
    }

    #[test]
    fn test_3_big_determinant() {
        assert_eq!(dbg!({
            let u = Matrix::from([[8., 5., -2., 4.], [4., 2.5, 20., 4.], [8., 5., 1., 4.], [28., -4., 17., 1.]]);
            u.determinant()
        }), 1032.);
    }
}