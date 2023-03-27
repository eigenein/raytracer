use glam::DVec3;
use serde::Deserialize;

use crate::ray::Ray;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Surface {
    Sphere { center: DVec3, radius: f64 },
}

impl Surface {
    pub fn hit_by(&self, ray: &Ray) -> Option<DVec3> {
        match self {
            Self::Sphere { center, radius } => {
                let oc = ray.origin - *center;
                let a = ray.direction.length_squared();
                let half_b = oc.dot(ray.direction);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant > 0.0 {
                    Some((ray.at((-half_b - discriminant.sqrt()) / a) - *center).normalize())
                } else {
                    None
                }
            }
        }
    }
}
