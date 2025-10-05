use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use super::Real;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: Real,
    pub y: Real,
}

impl Vec2 {
    pub fn new(x: Real, y: Real) -> Self {
        Self { x, y }
    }

    // --- Constants ---

    /// A constant for the zero vector `(0, 0)`.
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    /// Computes the magnitude (or Euclidean length) of the vector.
    ///
    /// The magnitude is calculated as the square root of the sum of the
    /// squares of its components: `sqrt(x^2 + y^2)`.
    ///
    /// For performance-critical code where you only need to compare lengths,
    /// consider using `magnitude_squared()` instead to avoid the expensive
    /// square root operation.
    pub fn magnitude(&self) -> Real {
        self.magnitude_squared().sqrt()
    }

    /// Computes the squared magnitude (or squared Euclidean length) of the vector.
    ///
    /// This is generally faster than calling `magnitude()` as it avoids the
    /// expensive square root operation. It is most useful when comparing
    /// the lengths of two vectors, as `a.magnitude_squared() < b.magnitude_squared()`
    /// is equivalent to `a.magnitude() < b.magnitude()`.
    pub fn magnitude_squared(&self) -> Real {
        self.x.powi(2) + self.y.powi(2)
    }

    /// Returns a new vector with the same direction and a magnitude of 1,
    /// also known as a **unit vector**.
    ///
    /// This method does not alter the original vector.
    ///
    /// The zero vector has a magnitude of zero and cannot be normalized.
    /// To ensure stability in simulations, this function will safely return a
    /// new zero vector in that case, preventing panics or `NaN` values.
    ///
    /// # See Also
    /// - `normalize()` for the in-place version of this method.
    #[must_use = "this returns a new vector, leaving the original unchanged"]
    pub fn normalized(&self) -> Self {
        let mag_sq = self.magnitude_squared();

        if mag_sq > 1e-9 {
            // Using magnitude_squared() and then a single sqrt() is often faster.
            let inv_mag = 1.0 / mag_sq.sqrt();
            return *self * inv_mag;
        }

        Self::ZERO
    }

    /// Normalizes the vector **in-place**, changing its magnitude to 1.
    ///
    /// This method **modifies** the vector it is called on.
    ///
    /// If the vector's magnitude is zero (or very close to zero), it will be
    /// set to the zero vector to prevent a division-by-zero panic.
    ///
    /// # See Also
    /// - `normalized()` for the version that returns a new vector.
    pub fn normalize(&mut self) {
        let mag_sq = self.magnitude_squared();

        if mag_sq > 1e-9 {
            let inv_mag = 1.0 / mag_sq.sqrt();
            *self *= inv_mag;
        } else {
            self.clear();
        }
    }

    /// Adds a scaled vector to this vector in-place.
    ///
    /// This operation is equivalent to `self = self + (other * scale)`.
    /// It modifies the vector on which it is called.
    pub fn add_scaled(&mut self, other: Self, scale: Real) {
        *self += other * scale;
    }

    /// Calculates the dot product of two vectors.
    ///
    /// The dot product is the sum of the products of the corresponding components.
    pub fn dot(&self, rhs: Self) -> Real {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Zero all the components of the vector.
    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    /// Flips all the components of the vector.
    pub fn invert(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
    }
}

// Component-wise multiplication
impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> Mul<T> for Vec2
where
    T: Into<Real>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs_real = rhs.into();
        Self {
            x: self.x * rhs_real,
            y: self.y * rhs_real,
        }
    }
}

impl<T> MulAssign<T> for Vec2
where
    T: Into<Real>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

// This allows `Real * Vec`
impl Mul<Vec2> for Real {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        // Simply reverse the order and reuse the existing implementation.
        rhs * self
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
