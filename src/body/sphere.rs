use glam::DVec3;
use serde::Deserialize;

use crate::ray::Ray;

#[derive(Deserialize)]
pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit_by(&self, ray: &Ray) -> Option<DVec3> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            Some((ray.at((-half_b - discriminant.sqrt()) / a) - self.center).normalize())
        } else {
            None
        }
    }
}
