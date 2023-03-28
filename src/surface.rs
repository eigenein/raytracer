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
    pub fn hit(&self, by_ray: &Ray, min_time: f64, max_time: f64) -> Option<Hit> {
        match *self {
            Self::Sphere { center, radius } => {
                let oc = by_ray.origin - center;
                let a = by_ray.direction.length_squared();
                let half_b = oc.dot(by_ray.direction);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant < 0.0 {
                    return None;
                }

                let mut time = (-half_b - discriminant.sqrt()) / a;
                if (time < min_time) || (time > max_time) {
                    time = (-half_b + discriminant.sqrt()) / a;
                    if (time < min_time) || (time > max_time) {
                        return None;
                    }
                }

                let location = by_ray.at(time);
                let outward_normal = (location - center) / radius;
                let normal = if outward_normal.dot(by_ray.direction) < 0.0 {
                    outward_normal
                } else {
                    -outward_normal
                };
                Some(Hit {
                    time,
                    location,
                    normal,
                })
            }
        }
    }
}
