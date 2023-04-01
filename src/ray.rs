use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    #[inline]
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    #[inline]
    pub fn by_two_points(from: DVec3, to: DVec3) -> Self {
        Self::new(from, to - from)
    }

    #[inline]
    pub fn at(&self, distance: f64) -> DVec3 {
        self.origin + self.direction * distance
    }
}
