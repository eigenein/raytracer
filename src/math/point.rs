use std::ops::{Add, Sub};

use glam::DVec3;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Point(DVec3);

impl Point {
    pub const ZERO: Self = Self::default();

    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(DVec3::new(x, y, z))
    }
}

impl const Default for Point {
    #[inline]
    fn default() -> Self {
        Self(DVec3::ZERO)
    }
}

impl const From<DVec3> for Point {
    #[inline]
    fn from(value: DVec3) -> Self {
        Self(value)
    }
}

impl Sub<Point> for Point {
    type Output = DVec3;

    #[inline]
    fn sub(self, rhs: Point) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<DVec3> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: DVec3) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Add<DVec3> for Point {
    type Output = Point;

    #[inline]
    fn add(self, rhs: DVec3) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<f64> for Point {
    type Output = Point;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<f64> for Point {
    type Output = Point;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        Self(self.0 - rhs)
    }
}
