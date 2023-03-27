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
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            Some((ray.at((-b - discriminant.sqrt()) / (2.0 * a)) - self.center).normalize())
        } else {
            None
        }
    }
}
