use crate::complex::Complex;

impl From<f32> for Complex {
    fn from(value: f32) -> Self {
        Self {
            r: value,
            i: 0.
        }
    }
}

pub enum ComplexError {
    CantCoerceComplexToRealDueToImaginaryPart
}

impl TryFrom<Complex> for f32 {
    type Error = ComplexError;

    fn try_from(value: Complex) -> Result<Self, Self::Error> {
        if value.i != 0. {
            Err(ComplexError::CantCoerceComplexToRealDueToImaginaryPart)
        } else {
            Ok(value.r)
        }
    }
}