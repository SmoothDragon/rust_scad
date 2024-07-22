use nalgebra as na;

use derive_more::*;

#[derive(Clone, Copy, PartialEq, Add, Sub, Mul, Neg)]
pub struct Real(pub f32);

impl Real {
    /// Positive Real MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: Real = Real(f32::MAX/1000.0);
}

impl std::fmt::Debug for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::fmt::Display for Real {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::ops::AddAssign for Real {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

// TODO: Macro to replace all this?
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

impl std::fmt::Display for Real2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

pub fn v2<X: Into<Real>, Y: Into<Real>>(x: X, y: Y) -> Real2 {
    Real2(nalgebra::vector![x.into(), y.into()])
}

impl<X: Into<Real>, Y: Into<Real>> From<(X,Y)> for Real2 {
    fn from(xy: (X,Y)) -> Real2 {
        v2(xy.0, xy.1)
    }
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

// TODO: Make this happen
// impl<T: Into<Real>> std::ops::Mul<Real2> for T {
    // type Output = Real2;
    // fn mul(self, rhs: Real2) -> Self::Output {
        // let value = self.into().0;
        // v2(rhs.0.x * value, rhs.0.y * value)
    // }
// }

#[derive(Debug, Clone, Copy, PartialEq, Add)]
pub struct Real3(pub na::Vector3<Real>);  // TODO: Remove pub na::

impl std::fmt::Display for Real3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

pub fn v3<X: Into<Real>, Y: Into<Real>, Z: Into<Real>>(x: X, y: Y, z: Z) -> Real3 {
    Real3(nalgebra::vector![x.into(), y.into(), z.into()])
}

impl std::ops::Mul<f32> for Real3 {
    type Output = Real3;
    fn mul(self, rhs: f32) -> Self::Output {
        v3(self.0.x * rhs, self.0.y * rhs, self.0.z * rhs)
    }
}


impl std::ops::Mul<Real3> for f32 {
    type Output = Real3;
    fn mul(self, rhs: Real3) -> Self::Output {
        v3(rhs.0.x * self, rhs.0.y * self, rhs.0.z * self)
    }
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

    #[test]
    fn test_v2_mul() {
        assert_eq!(format!("{}", v2(1.,2.)*3.), "[3, 6]");
        assert_eq!(format!("{}", 3. * v2(1.,2.)), "[3, 6]");
    }

    #[test]
    fn test_v3_mul() {
        assert_eq!(format!("{}", v3(1.,2., 4)*3.), "[3, 6, 12]");
        assert_eq!(format!("{}", 8. * v3(1.,2., 4)), "[8, 16, 32]");
    }

    #[test]
    fn test_into_real2() {
        assert_eq!(Real2::from( (5_i32, 10_i32) ), v2(5., 10.));
        assert_eq!(Real2::from( (5_i32, 10_u64) ), v2(5., 10.));
        assert_eq!(Real2::from( (5.0_f32, 10_u64) ), v2(5., 10.));
    }

}
