use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
    pub refractive_index: f64,
}

impl Ray {
    pub fn by_two_points(from: DVec3, to: DVec3) -> Self {
        Self {
            origin: from,
            direction: to - from,
            refractive_index: 1.0,
        }
    }

    pub fn at(&self, distance: f64) -> DVec3 {
        self.origin + self.direction * distance
    }

    pub fn normalize(&mut self) {
        self.direction = self.direction.normalize();
    }
}
