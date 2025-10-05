pub type Real = f32;

// impl Real {
//     pub const MAX: Self = Self(f32::MAX);
//     // A very small number for floating-point comparisons.
//     pub const EPSILON: Self = Self(1e-6);
//
//     pub fn pow(&self, n: Self) -> Self {
//         Self(self.0.powf(n.0))
//     }
//
//     pub fn abs(&self) -> Self {
//         Self(self.0.abs())
//     }
// }

// impl<T> From<T> for Real
// where
//     T: Into<f32>,
// {
//     fn from(value: T) -> Self {
//         Real(value.into())
//     }
// }
//
// impl PartialEq for Real {
//     fn eq(&self, other: &Self) -> bool {
//         (self.0 - other.0).abs() < Real::EPSILON.0
//     }
// }
//
// impl<T> PartialEq<T> for Real
// where
//     T: Into<f32> + Copy,
// {
//     fn eq(&self, other: &T) -> bool {
//         (self.0 - (*other).into()).abs() < Real::EPSILON.0
//     }
// }
//
// impl PartialOrd for Real {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         if (self.0 - other.0).abs() < Real::EPSILON.0 {
//             Some(std::cmp::Ordering::Equal)
//         } else {
//             self.0.partial_cmp(&other.0)
//         }
//     }
// }
//
// impl<T> PartialOrd<T> for Real
// where
//     T: Into<f32> + Copy,
// {
//     fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
//         let other_f32: f32 = (*other).into();
//         if (self.0 - other_f32).abs() < Real::EPSILON.0 {
//             Some(std::cmp::Ordering::Equal)
//         } else {
//             self.0.partial_cmp(&other_f32)
//         }
//     }
// }
//
// impl Add for Real {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Self(self.0 + rhs.0)
//     }
// }
//
// impl AddAssign for Real {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs
//     }
// }
//
// impl Sub for Real {
//     type Output = Self;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         Self(self.0 - rhs.0)
//     }
// }
//
// impl SubAssign for Real {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = *self - rhs
//     }
// }
//
// impl<T> Mul<T> for Real
// where
//     T: Into<Real>,
// {
//     type Output = Self;
//
//     fn mul(self, rhs: T) -> Self::Output {
//         Self(self.0 * rhs.into().0)
//     }
// }
//
// impl<T> MulAssign<T> for Real
// where
//     T: Into<Real>,
// {
//     fn mul_assign(&mut self, rhs: T) {
//         self.0 *= rhs.into().0;
//     }
// }
//
// impl Neg for Real {
//     type Output = Self;
//
//     fn neg(self) -> Self::Output {
//         Self(-self.0)
//     }
// }
//
// impl<T> Div<T> for Real
// where
//     T: Into<Real>,
// {
//     type Output = Self;
//
//     fn div(self, rhs: T) -> Self::Output {
//         Self(self.0 / rhs.into().0)
//     }
// }
//
// impl<T> DivAssign<T> for Real
// where
//     T: Into<Real>,
// {
//     fn div_assign(&mut self, rhs: T) {
//         self.0 /= rhs.into().0;
//     }
// }
