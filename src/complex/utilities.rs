use std::fmt::{Debug, Display, Formatter};
use super::Complex;

impl Default for Complex {
    fn default() -> Self {
        Self {
            r: 0.0,
            i: 0.0,
        }
    }
}

impl Complex {
    pub fn new(r: f32, i: f32) -> Self {
        Self { r, i }
    }

    pub fn from_i(i: f32) -> Self {
        Self {
            r: 0.,
            i
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