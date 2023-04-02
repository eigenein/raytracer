use glam::DVec3;

use crate::math::point::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: DVec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point, direction: DVec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    #[inline]
    pub fn by_two_points(from: Point, to: Point) -> Self {
        Self::new(from, to - from)
    }

    #[inline]
    pub fn at(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }
}
