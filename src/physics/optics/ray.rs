use crate::math::point3::Point3;
use crate::math::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    #[inline]
    pub fn by_two_points(from: Point3, to: Point3) -> Self {
        Self::new(from, to - from)
    }

    #[inline]
    pub fn at(&self, distance: f64) -> Point3 {
        self.origin + self.direction * distance
    }
}
