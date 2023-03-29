use glam::DVec3;

use crate::hit::Hit;
use crate::math::random_unit_vector;

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

    pub fn at(&self, time: f64) -> DVec3 {
        self.origin + self.direction * time
    }

    pub fn reflect(&self, hit: &Hit, fuzz: f64) -> Self {
        let mut direction = self.direction - hit.normal * (2.0 * self.direction.dot(hit.normal));
        if fuzz != 0.0 {
            direction += random_unit_vector() * fuzz * direction.length();
        }
        Self {
            origin: hit.location,
            direction,
            refractive_index: self.refractive_index,
        }
    }

    /// See also: <https://en.wikipedia.org/wiki/Lambertian_reflectance>.
    pub fn reflect_diffused(&self, hit: &Hit) -> Self {
        Self {
            origin: hit.location,
            direction: hit.normal + random_unit_vector(),
            refractive_index: self.refractive_index,
        }
    }

    /// See also: <https://physics.stackexchange.com/a/436252/11966>.
    pub fn refract(&self, hit: &Hit, inner_index: f64) -> Self {
        let mu = if hit.from_outside {
            self.refractive_index / inner_index
        } else {
            inner_index / self.refractive_index
        };
        let incident_direction = self.direction.normalize();
        let incident_dot_normal = incident_direction.dot(hit.normal);
        let direction = (incident_direction - hit.normal * incident_dot_normal) * mu
            - hit.normal * (1.0 - mu.powi(2) * (1.0 - incident_dot_normal.powi(2))).sqrt();
        Self {
            origin: hit.location,
            direction,
            refractive_index: inner_index,
        }
    }
}
