use glam::DVec3;

use crate::hit::Hit;
use crate::math::{random_unit_vector, reflect};

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn by_two_points(from: DVec3, to: DVec3) -> Self {
        Self {
            origin: from,
            direction: to - from,
        }
    }

    pub fn at(&self, time: f64) -> DVec3 {
        self.origin + self.direction * time
    }

    pub fn reflect(&self, hit: &Hit, fuzz: f64) -> Self {
        let mut direction = reflect(self.direction, hit.normal);
        if fuzz != 0.0 {
            direction += random_unit_vector() * fuzz * direction.length();
        }
        Self {
            origin: hit.location,
            direction,
        }
    }

    /// See also: <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
    pub fn reflect_diffused(hit: &Hit) -> Self {
        Self {
            origin: hit.location,
            direction: hit.normal + random_unit_vector(),
        }
    }
}
