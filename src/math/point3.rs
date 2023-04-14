use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::vec3::Vec3;

#[derive(Debug, Copy, Clone, Deserialize, JsonSchema)]
pub struct Point3(Vec3);

impl Point3 {
    #[allow(dead_code)]
    pub const ONE: Self = Self(Vec3::ONE);
    #[allow(dead_code)]
    pub const ZERO: Self = Self(Vec3::ZERO);

    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    #[inline]
    pub const fn is_infinite(&self) -> bool {
        self.0.x.is_infinite() && self.0.y.is_infinite() && self.0.z.is_infinite()
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(clippy::derivable_impls)]
impl const Default for Point3 {
    #[inline]
    fn default() -> Self {
        Self(Vec3::default())
    }
}

impl const From<Vec3> for Point3 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl const Sub<Point3> for Point3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Point3) -> Self::Output {
        self.0 - rhs.0
    }
}

impl const Sub<Vec3> for Point3 {
    type Output = Point3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl const Add<Vec3> for Point3 {
    type Output = Point3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl const Add<f64> for Point3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl const Sub<f64> for Point3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
    }
}
