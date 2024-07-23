use std::fmt::{Debug, Display, Formatter};
use crate::quaternion::Quaternion;
use crate::utils::Unit;
use crate::vector::Vec3;
use crate::exercises::ex06::Ex06crossProduct;

impl Quaternion {

    pub fn r(&self) -> f32 { self.r }
    pub fn i(&self) -> f32 { self.i }
    pub fn j(&self) -> f32 { self.j }
    pub fn k(&self) -> f32 { self.k }

    pub fn r_mut(&mut self) -> &mut f32 { &mut self.r }
    pub fn i_mut(&mut self) -> &mut f32 { &mut self.i }
    pub fn j_mut(&mut self) -> &mut f32 { &mut self.j }
    pub fn k_mut(&mut self) -> &mut f32 { &mut self.k }

    pub fn conjugate(&self) -> Self {
        Self {
            r: self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.r * rhs.r + self.i * rhs.i + self.j * rhs.j + self.k * rhs.k
    }

    pub fn inverse(&self) -> Self {
        let l = self.dot(&self);
        if l == 0. {
            *self
        } else if l != 1. {
            (*self * (1. / l)).conjugate()
        } else {
            self.conjugate()
        }
    }

    pub fn norm(&self) -> f32 {
        let l = self.dot(&self);
        if l == 0. || l == 1. {
            l
        } else {
            l.sqrt()
        }
    }

    pub fn from_look_at(look: Vec3, up: Vec3) -> Self {
        let right = up.cross_product(&look).normalized();
        let up = look.cross_product(&right);
        let t1 = 1. + right[0] - up[1] - look[2];
        let t2 = 1. - right[0] + up[1] - look[2];
        let t3 = 1. - right[0] - up[1] + look[2];
        if t1 + t2 + t3 < 0. {
            Self::unit()
        } else if t1 > t2 && t1 > t3 {
            let s = t1.sqrt() * 2.;
            Self {
                r: (up[2] - look[1]) / s,
                i: s / 4.,
                j: (up[0] + right[1]) / s,
                k: (look[0] + right[2]) / s,
            }
        } else if t2 > t1 && t2 > t3 {
            let s = t2.sqrt() * 2.;
            Self {
                r: (look[0] - right[2]) / s,
                i: (up[0] + right[1]) / s,
                j: s / 4.,
                k: (look[1] + up[2]) / s,
            }
        } else {
            let s = t3.sqrt() * 2.;
            Self {
                r: (right[1] - up[0]) / s,
                i: (look[0] + right[2]) / s,
                j: (look[1] + up[2]) / s,
                k: s / 4.,
            }
        }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn plus(val: f32) -> &'static str {
            if val >= 0. { "+" } else { "" }
        }

        match (self.r == 0., self.i == 0., self.j == 0., self.k == 0.) {
            (_, true, true, true) => f.write_fmt(format_args!("{}", self.r)),
            (true, false, true, true) => f.write_fmt(format_args!("{}i", self.i)),
            (false, false, true, true) => f.write_fmt(format_args!("{}{}{}i", self.r, plus(self.i), self.i)),
            (true, true, false, true) => f.write_fmt(format_args!("{}j", self.j)),
            (false, true, false, true) => f.write_fmt(format_args!("{}{}{}j", self.r, plus(self.j), self.j)),
            (false, false, false, true) => f.write_fmt(format_args!("{}{}{}i{}{}j", self.r, plus(self.i), self.i, plus(self.j), self.j)),
            (true, true, false, false) => f.write_fmt(format_args!("{}j{}{}k", self.j, plus(self.k), self.k)),
            (true, true, true, false) => f.write_fmt(format_args!("{}k", self.k)),
            (false, true, true, false) => f.write_fmt(format_args!("{}{}{}k", self.r, plus(self.k), self.k)),
            (false, false, true, false) => f.write_fmt(format_args!("{}{}{}i{}{}k", self.r, plus(self.i), self.i, plus(self.k), self.k)),
            (false, false, false, false) => f.write_fmt(format_args!("{}{}{}i{}{}j{}{}k", self.r, plus(self.i), self.i, plus(self.j), self.j, plus(self.k), self.k)),
            (true, false, false, false) => f.write_fmt(format_args!("{}i{}{}j{}{}k", self.i, plus(self.j), self.j, plus(self.k), self.k)),
            (true, false, true, false) => f.write_fmt(format_args!("{}i{}{}k", self.i, plus(self.k), self.k)),
            (true, false, false, true) => f.write_fmt(format_args!("{}i{}{}j", self.i, plus(self.j), self.j)),
            (false, true, false, false) => f.write_fmt(format_args!("{}{}{}j{}{}k", self.r, plus(self.j), self.j, plus(self.k), self.k)),
        }
    }
}

impl Debug for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{{r: {}, i: {}, j: {}, k: {}}}", self.r, self.i, self.j, self.k))
    }
}

impl Clone for Quaternion {
    fn clone(&self) -> Self {
        Self {
            r: self.r,
            i: self.i,
            j: self.j,
            k: self.k
        }
    }
}

impl Copy for Quaternion {}