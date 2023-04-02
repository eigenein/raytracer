use std::ops::{Add, Sub};

use glam::DVec3;
use serde::Deserialize;

#[derive(Default, Debug, Copy, Clone, Deserialize)]
pub struct Point(DVec3);

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
