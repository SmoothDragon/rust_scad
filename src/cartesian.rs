use std::ops::*;

use derive_more::*;

pub const PI: X = X(std::f32::consts::PI);

#[derive(Clone, Copy, PartialEq, Neg)]
pub struct X(pub f32);

impl X {
    /// Positive X MAX is lower since it is used for super large objects that could be shifted or rotated.
    pub const MAX: X = X(f32::MAX/1000.0);

    pub fn atan(self) -> Self {
        X(self.0.atan())
    }

    pub fn cos(self) -> Self {
        X(self.0.cos())
    }
}

impl std::fmt::Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::fmt::Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::ops::AddAssign for X {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

impl std::ops::SubAssign for X {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0);
    }
}

// TODO: Macro to replace all this?
impl From<u32> for X {
    fn from(i: u32) -> X {
        X(i as f32)
    }
}

impl From<i32> for X {
    fn from(i: i32) -> X {
        X(i as f32)
    }
}

impl From<u64> for X {
    fn from(i: u64) -> X {
        X(i as f32)
    }
}

impl From<i64> for X {
    fn from(i: i64) -> X {
        X(i as f32)
    }
}

impl From<f32> for X {
    fn from(f: f32) -> X {
        X(f as f32)
    }
}

impl From<f64> for X {
    fn from(f: f64) -> X {
        X(f as f32)
    }
}

impl<IX: Into<X>> std::ops::Mul<IX> for X {
    type Output = X;
    fn mul(self, other: IX) -> Self::Output {
        X(self.0 * other.into().0)
    }
}

impl<IX: Into<X>> std::ops::Div<IX> for X {
    type Output = X;
    fn div(self, other: IX) -> Self::Output {
        X(self.0 / other.into().0)
    }
}

impl<IX: Into<X>> std::ops::Add<IX> for X {
    type Output = X;
    fn add(self, other: IX) -> Self::Output {
        X(self.0 + other.into().0)
    }
}

impl<IX: Into<X>> std::ops::Sub<IX> for X {
    type Output = X;
    fn sub(self, other: IX) -> Self::Output {
        X(self.0 - other.into().0)
    }
}

impl std::ops::Mul<X> for f32 {
    type Output = X;
    fn mul(self, other: X) -> X {
        X(self * other.0)
    }
}

impl std::ops::Mul<X> for i32 {
    type Output = X;
    fn mul(self, other: X) -> X {
        X((self as f32) * other.0)
    }
}

impl std::ops::Div<X> for f32 {
    type Output = X;
    fn div(self, other: X) -> X {
        X(self / other.0)
    }
}

impl std::ops::Div<X> for i32 {
    type Output = X;
    fn div(self, other: X) -> X {
        X((self as f32) / other.0)
    }
}

impl std::ops::Sub<X> for f32 {
    type Output = X;
    fn sub(self, other: X) -> X {
        X(self - other.0)
    }
}

impl std::ops::Sub<X> for i32 {
    type Output = X;
    fn sub(self, other: X) -> X {
        X((self as f32) - other.0)
    }
}

impl std::ops::Add<X> for f32 {
    type Output = X;
    fn add(self, other: X) -> X {
        X(self + other.0)
    }
}

impl std::ops::Add<X> for i32 {
    type Output = X;
    fn add(self, other: X) -> X {
        X((self as f32) + other.0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Add, Neg)]
pub struct XY(pub f32, pub f32); 

impl std::fmt::Display for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format!("[{}, {}]", &self.0, &self.1))
        // write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
    }
}

impl From<XY> for [f32; 2] {
    fn from(xy: XY) -> [f32; 2] {
        [xy.0, xy.1]
    }
}

pub fn v2<IX: Into<X>, IY: Into<X>>(x: IX, y: IY) -> XY {
    XY(x.into().0, y.into().0)
}

impl<IX: Into<X>, IY: Into<X>> From<(IX,IY)> for XY {
    fn from(xy: (IX,IY)) -> XY {
        v2(xy.0, xy.1)
    }
}

/// Generalized multiplication on the right is possible
impl<IX: Into<X>> std::ops::Mul<IX> for XY {
    type Output = XY;
    fn mul(self, other: IX) -> Self::Output {
        let y: f32 = other.into().0;
        // v2(self.0.x * other.clone().into().0, self.0.y * other.into().0)
        v2(self.0 * y, self.1 * y)
    }
}

/// Generalized multiplication on the left is not currently possible
/// Each type must be specified individually
/// TODO: This should become a macro
impl std::ops::Mul<XY> for f32 {
    type Output = XY;
    fn mul(self, rhs: XY) -> Self::Output {
        v2(rhs.0 * self as f32, rhs.1 * self as f32)
    }
}

impl std::ops::Mul<XY> for i32 {
    type Output = XY;
    fn mul(self, rhs: XY) -> Self::Output {
        v2(rhs.0 * self as f32, rhs.1 * self as f32)
    }
}


impl Sub for XY {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

/// Multiplication treats XY as a complex number
impl Mul for XY {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0 - self.1 * other.1, 
             self.0 * other.1 + self.1 * other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Add)]
// pub struct XYZ(pub na::Vector3<X>);  // TODO: Remove pub na::
pub struct XYZ(pub f32, pub f32, pub f32);

impl std::fmt::Display for XYZ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "{}", format!("{:?}", &self.0).replace(r"[[", r"[").replace("]]", "]"))
        write!(f, "{}", format!("[{}, {}, {}]", &self.0, &self.1, &self.2))
    }
}

pub fn v3<IX: Into<X>, IY: Into<X>, IZ: Into<X>>(x: IX, y: IY, z: IZ) -> XYZ {
    // XYZ(nalgebra::vector![x.into(), y.into(), z.into()])
    XYZ(x.into().0, y.into().0, z.into().0)
}

impl From<XY> for XYZ {
    fn from(xy: XY) -> XYZ {
        v3(xy.0, xy.1, 0)
    }
}

impl std::ops::Mul<f32> for XYZ {
    type Output = XYZ;
    fn mul(self, rhs: f32) -> Self::Output {
        v3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}


impl std::ops::Mul<XYZ> for f32 {
    type Output = XYZ;
    fn mul(self, rhs: XYZ) -> Self::Output {
        v3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_i32() {
        assert_eq!(X::from(5_i32), X(5.));
        assert_eq!(X::from(i32::MAX), X(2147483600.0));
    }

    #[test]
    fn test_into_real() {
        assert_eq!(<i32 as Into<X>>::into(5), X(5.));
        assert_eq!(<u32 as Into<X>>::into(5), X(5.));
        assert_eq!(<f64 as Into<X>>::into(5.0), X(5.));
    }

    #[test]
    fn test_into_array_f32() {
        assert_eq!(Into::<[f32; 2]>::into(XY(1.0, 2.0)), [1.0, 2.0]);
    }

    #[test]
    fn test_real_mul() {
        assert_eq!(X(5.) * 2., X(10.));
        assert_eq!(X(5.) * 2, X(10.));
        assert_eq!(2. * X(5.), X(10.));
        assert_eq!(2 * X(5.), X(10.));
    }

    #[test]
    fn test_v2_mul() {
        assert_eq!(format!("{}", v2(1.,2.)*3.), "[3, 6]");
        assert_eq!(format!("{}", 3. * v2(1.,2.)), "[3, 6]");
        assert_eq!(format!("{}", v2(1.,2.)*3), "[3, 6]");
        assert_eq!(format!("{}", 3 * v2(1.,2.)), "[3, 6]");
    }

    #[test]
    fn test_v2_v2_mul() {
        assert_eq!(format!("{}", v2(1.,2.)*v2(1, -2)), "[5, 0]");
    }

    #[test]
    fn test_v3_mul() {
        assert_eq!(format!("{}", v3(1.,2., 4)*3.), "[3, 6, 12]");
        assert_eq!(format!("{}", 8. * v3(1.,2., 4)), "[8, 16, 32]");
    }

    #[test]
    fn test_into_real2() {
        assert_eq!(XY::from( (5_i32, 10_i32) ), v2(5., 10.));
        assert_eq!(XY::from( (5_i32, 10_u64) ), v2(5., 10.));
        assert_eq!(XY::from( (5.0_f32, 10_u64) ), v2(5., 10.));
    }

}
