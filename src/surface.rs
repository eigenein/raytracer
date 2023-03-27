use glam::DVec3;
use serde::Deserialize;

use crate::hit::Hit;
use crate::ray::Ray;

/// Surface that is being rendered.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Surface {
    Sphere { center: DVec3, radius: f64 },
}

impl Surface {
    /// Calculate a hit of the surface by the specified ray.
    pub fn hit(&self, by_ray: &Ray) -> Option<Hit> {
        match *self {
            Self::Sphere { center, radius } => {
                let oc = by_ray.origin - center;
                let a = by_ray.direction.length_squared();
                let half_b = oc.dot(by_ray.direction);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                (discriminant > 0.0).then(|| {
                    let time = (-half_b - discriminant.sqrt()) / a;
                    let location = by_ray.at(time);
                    let normal = (location - center) / radius;
                    Hit {
                        time,
                        location,
                        normal,
                    }
                })
            }
        }
    }
}
