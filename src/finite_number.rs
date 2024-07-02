extern crate nalgebra as na;

use std::ops::*;
use std::fmt;
use derive_more::*;

#[derive(Clone, Copy, PartialEq, Add, Sub, Mul, Neg)]
pub struct Real(pub f32);

impl Real {
    /// Positive Real MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: Real = Real(f32::MAX/1000.0);
}

impl fmt::Debug for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl AddAssign for Real {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

impl From<u32> for Real {
    fn from(i: u32) -> Real {
        Real(i as f32)
    }
}

impl From<i32> for Real {
    fn from(i: i32) -> Real {
        Real(i as f32)
    }
}

impl From<u64> for Real {
    fn from(i: u64) -> Real {
        Real(i as f32)
    }
}

impl From<i64> for Real {
    fn from(i: i64) -> Real {
        Real(i as f32)
    }
}

impl From<f32> for Real {
    fn from(f: f32) -> Real {
        Real(f as f32)
    }
}

impl From<f64> for Real {
    fn from(f: f64) -> Real {
        Real(f as f32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add)]
pub struct Real2(pub na::Vector2<Real>); // TODO: Remove pub na::

impl fmt::Display for Real2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

pub fn v2<X: Into<Real>, Y: Into<Real>>(x: X, y: Y) -> Real2 {
    Real2(nalgebra::vector![x.into(), y.into()])
}

impl std::ops::Mul<f32> for Real2 {
    type Output = Real2;
    fn mul(self, rhs: f32) -> Self::Output {
        v2(self.0.x * rhs, self.0.y * rhs)
    }
}


impl std::ops::Mul<Real2> for f32 {
    type Output = Real2;
    fn mul(self, rhs: Real2) -> Self::Output {
        v2(rhs.0.x * self, rhs.0.y * self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add, Mul)]
pub struct Real3(pub na::Vector3<Real>);  // TODO: Remove pub na::

impl fmt::Display for Real3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

pub fn v3<X: Into<Real>, Y: Into<Real>, Z: Into<Real>>(x: X, y: Y, z: Z) -> Real3 {
    Real3(nalgebra::vector![x.into(), y.into(), z.into()])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(Real::from(5_i32), Real(5.));
        assert_eq!(Real::from(i32::MAX), Real(2147483600.0));
    }

    #[test]
    fn test_into_real() {
        assert_eq!(<i32 as Into<Real>>::into(5), Real(5.));
        assert_eq!(<u32 as Into<Real>>::into(5), Real(5.));
        assert_eq!(<f64 as Into<Real>>::into(5.0), Real(5.));
    }

    // TODO:
    #[test]
    fn test_v2_mul() {
        assert_eq!(format!("{}", v2(1.,2.)*3.), "[3, 6]");
        assert_eq!(format!("{}", 3. * v2(1.,2.)), "[3, 6]");
    }
}
