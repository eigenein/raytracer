use glam::DVec3;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn at(&self, time: f64) -> DVec3 {
        self.origin + self.direction * time
    }
}
