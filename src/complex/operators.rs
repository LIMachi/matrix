use std::ops::{Add, Div, Mul, Neg, Sub};
use super::Complex;

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i
    }
}

impl PartialEq<f32> for Complex {
    fn eq(&self, other: &f32) -> bool {
        self.i == 0. && self.r == *other
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

impl Add<f32> for Complex {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r + rhs,
            i: self.i
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
        }
    }
}

impl Sub<f32> for Complex {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r - rhs,
            i: self.i
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            r: -self.r,
            i: -self.i
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r
        }
    }
}

impl Mul<f32> for Complex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            i: self.i * rhs
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let div = rhs.r * rhs.r + rhs.i * rhs.i;
        Self {
            r: (self.r * rhs.r + self.i * rhs.i) / div,
            i: (self.i * rhs.r - self.r * rhs.i) / div
        }
    }
}

impl Div<f32> for Complex {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r / rhs,
            i: self.i / rhs
        }
    }
}