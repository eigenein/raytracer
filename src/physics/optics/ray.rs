use crate::math::point::Point;
use crate::math::vec::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point, direction: Vec3) -> Self {
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
