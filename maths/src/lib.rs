pub mod vector;
pub mod matrix;
pub mod complex;
pub mod quaternion;
pub mod utils;

pub mod prelude {
    pub use crate::{
        utils::{Unit, Norm, Lerp},
        vector::{Vector, DynVec, Vec3, Vec4},
        matrix::{Matrix, DynMat, Mat3, Mat4}
    };
}