use crate::math::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    #[inline]
    pub fn with_two_points(from: Vec3, to: Vec3) -> Self {
        Self::new(from, to - from)
    }

    #[inline]
    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction * distance
    }
}
