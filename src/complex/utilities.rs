use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use crate::complex::Complex;

impl Default for Complex {
    fn default() -> Self {
        Self {
            r: 0.0,
            i: 0.0,
        }
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{r: {}, i: {}}}", self.r, self.i))
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.i == 0. {
            std::fmt::Display::fmt(&self.r, f)
        } else if self.r == 0. {
            f.write_fmt(format_args!("{}i", self.i))
        } else {
            f.write_fmt(format_args!("{}+{}i", self.r, self.i))
        }
    }
}

impl Clone for Complex {
    fn clone(&self) -> Self {
        Self {
            r: self.r,
            i: self.i
        }
    }
}

impl Copy for Complex {}

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

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            i: self.i - rhs.i
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