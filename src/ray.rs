use glam::DVec3;

use crate::math::point::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: DVec3,
    pub wavelength: f64,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point, direction: DVec3, wavelength: f64) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
            wavelength,
        }
    }

    #[inline]
    pub fn by_two_points(from: Point, to: Point, wavelength: f64) -> Self {
        Self::new(from, to - from, wavelength)
    }

    #[inline]
    pub fn at(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }
}
