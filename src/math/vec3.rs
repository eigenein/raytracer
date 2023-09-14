use std::f64::consts::TAU;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::sequence::Sequence;
use crate::math::vec2::Vec2;

#[repr(simd)]
#[derive(Copy, Clone, Debug, Deserialize, JsonSchema)]
#[must_use]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        self + Self::splat(rhs)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        self - Self::splat(rhs)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vec3 {
    pub const ONE: Self = Self::splat(1.0);
    pub const ZERO: Self = Self::splat(0.0);

    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn splat(value: f64) -> Self {
        Self { x: value, y: value, z: value }
    }

    /// Sample a unit vector from a uniform 2D-sequence.
    pub fn sample_unit_vector(sequence: &mut impl Sequence<Vec2>) -> Self {
        let sample = sequence.next();
        let theta = TAU * sample.x;
        let z = 2.0 * sample.y - 1.0;
        let scale = (1.0 - z * z).sqrt();
        let (theta_sin, theta_cos) = theta.sin_cos();
        Self {
            x: scale * theta_cos,
            y: scale * theta_sin,
            z,
        }
    }

    #[inline]
    #[must_use]
    pub const fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    #[must_use]
    pub const fn length_squared(self) -> f64 {
        self.dot(self)
    }

    #[inline]
    #[must_use]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self / self.length()
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    #[inline]
    #[must_use]
    pub fn max_element(self) -> f64 {
        self.x.max(self.y.max(self.z))
    }

    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    #[inline]
    #[must_use]
    pub fn min_element(self) -> f64 {
        self.x.min(self.y.min(self.z))
    }

    #[inline]
    pub const fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    /// Rotate the vector around another **unit** vector:
    /// <https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula>.
    #[inline]
    pub fn rotate_about(self, axis: Self, angle: f64) -> Self {
        self.assert_normalized();
        let (angle_sin, angle_cos) = angle.sin_cos();
        self * angle_cos + axis.cross(self) * angle_sin + axis.dot(self) * (1.0 - angle_cos) * axis
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
            z: self.z.clamp(min.z, max.z),
        }
    }

    #[inline]
    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    #[inline]
    pub fn powf(self, power: f64) -> Self {
        Self {
            x: self.x.powf(power),
            y: self.y.powf(power),
            z: self.z.powf(power),
        }
    }

    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool {
        (self.x - rhs.x).abs() <= max_abs_diff
            && (self.y - rhs.y).abs() <= max_abs_diff
            && (self.z - rhs.z).abs() <= max_abs_diff
    }

    #[inline]
    pub fn assert_normalized(self) {
        const THRESHOLD: f64 = 10000.0 * f64::EPSILON;
        assert!(
            (self.length_squared() - 1.0).abs() <= THRESHOLD,
            "expected normalized vector, actual length²: `{}`",
            self.length_squared(),
        );
    }

    #[inline]
    pub fn reflect_about(self, normal: Self) -> Self {
        normal.assert_normalized();
        self - 2.0 * self.dot(normal) * normal
    }

    #[inline]
    #[must_use]
    pub const fn is_infinite(self) -> bool {
        self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use approx::*;

    use super::*;
    use crate::math::sequence::Halton2;

    #[test]
    fn random_unit_vector_ok() {
        assert_abs_diff_eq!(Vec3::sample_unit_vector(&mut Halton2::new(2, 3)).length(), 1.0);
    }
}
