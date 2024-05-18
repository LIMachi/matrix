use crate::complex::Complex;
use crate::matrix::Mat4;
use crate::unit::Unit;
use crate::vector::{Vec3, Vec4};
use super::Quaternion;

#[derive(Debug)]
pub enum QuaternionError {
    CantCoerceQuaternionToRealDueToImaginaryParts,
    CantCoerceQuaternionToComplexDueToImaginaryParts,
}

impl From<f32> for Quaternion {
    fn from(value: f32) -> Self {
        Self {
            r: value.into(),
            i: 0.,
            j: 0.,
            k: 0.
        }
    }
}

impl TryFrom<Quaternion> for f32 {
    type Error = QuaternionError;

    fn try_from(value: Quaternion) -> Result<Self, Self::Error> {
        if value.i != 0. || value.j != 0. || value.k != 0. {
            Err(QuaternionError::CantCoerceQuaternionToRealDueToImaginaryParts)
        } else {
            Ok(value.r)
        }
    }
}

impl From<Complex> for Quaternion {
    fn from(value: Complex) -> Self {
        Self {
            r: value.r,
            i: value.i,
            j: 0.,
            k: 0.
        }
    }
}

impl TryFrom<Quaternion> for Complex {
    type Error = QuaternionError;

    fn try_from(value: Quaternion) -> Result<Self, Self::Error> {
        if value.j != 0. || value.k != 0. {
            Err(QuaternionError::CantCoerceQuaternionToComplexDueToImaginaryParts)
        } else {
            Ok(Self { r: value.r, i: value.i })
        }
    }
}

impl From<Vec3> for Quaternion {
    fn from(axis: Vec3) -> Self {
        let sqr = axis.dot(&axis);
        if sqr == 0. {
            Self::unit()
        } else {
            let angle = 0.5f32.to_radians();
            let s = if sqr != 1. {
                angle.sin() / sqr.sqrt()
            } else {
                angle.sin()
            };
            Self {
                r: angle.cos(),
                i: axis[0] * s,
                j: axis[1] * s,
                k: axis[2] * s,
            }
        }
    }
}

impl From<(Vec3, f32)> for Quaternion {
    fn from((axis, angle): (Vec3, f32)) -> Self {
        let sqr = axis.dot(&axis);
        let angle = angle / 2.;
        if sqr == 0. {
            Self::unit()
        } else {
            let s = if sqr != 1. {
                angle.sin() / sqr.sqrt()
            } else {
                angle.sin()
            };
            Self {
                r: angle.cos(),
                i: axis[0] * s,
                j: axis[1] * s,
                k: axis[2] * s,
            }
        }
    }
}

impl From<Quaternion> for Mat4 {
    fn from(mut value: Quaternion) -> Self {
        let sqr = {
            let t = Vec4::from([value.r, value.i, value.j, value.k]);
            t.dot(&t)
        };
        if sqr != 0. {
            if sqr != 1. {
                value = value / sqr.sqrt();
            }
            Mat4::from([[
                1. - 2. * (value.j * value.j + value.k * value.k),
                2. * (value.i * value.j - value.r * value.k),
                2. * (value.r * value.j + value.i * value.k),
                0.
            ], [
                2. * (value.r * value.k + value.i * value.j),
                1. - 2. * (value.i * value.i + value.k * value.k),
                2. * (value.j * value.k - value.r * value.i),
                0.
            ], [
                2. * (value.i * value.k - value.r * value.j),
                2. * (value.r * value.i + value.j * value.k),
                1. - 2. * (value.i * value.i + value.j * value.j),
                0.
            ], [0., 0., 0., 1.]])
        } else {
            Mat4::identity()
        }
    }
}