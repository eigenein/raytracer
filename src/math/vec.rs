use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use schemars::JsonSchema;
use serde::Deserialize;

#[repr(simd)]
#[derive(Copy, Clone, Debug, Deserialize, JsonSchema)]
#[must_use]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl const Default for Vec3 {
    #[inline]
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl const Mul<f64> for Vec3 {
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

impl const Mul<Vec3> for f64 {
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

impl const Add for Vec3 {
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

impl const Sub for Vec3 {
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

impl const Div<f64> for Vec3 {
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

impl const Div for Vec3 {
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

impl const Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl const Add<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        self + Self::splat(rhs)
    }
}

impl const Sub<f64> for Vec3 {
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

    pub fn random_unit_vector() -> Self {
        loop {
            let vector =
                Vec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, fastrand::f64() - 0.5);
            if vector.length_squared() <= 0.25 {
                return vector.normalize();
            }
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
        assert!((axis.length_squared() - 1.0).abs() <= f64::EPSILON);
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
}
