use std::ops::Range;

use glam::DVec3;
use serde::Deserialize;

use crate::hit::Hit;
use crate::ray::Ray;

/// Surface that is being rendered.
#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Surface {
    Sphere(Sphere),
}

impl Surface {
    /// Calculate a hit of the surface by the specified ray.
    pub fn hit(&self, by_ray: &Ray, time_range: Range<f64>) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(by_ray, time_range),
        }
    }
}

#[derive(Deserialize)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
}

impl Sphere {
    pub fn hit(&self, by_ray: &Ray, time_range: Range<f64>) -> Option<Hit> {
        let oc = by_ray.origin - self.center;
        let a = by_ray.direction.length_squared();
        let half_b = oc.dot(by_ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut time = (-half_b - discriminant_sqrt) / a;
        if !time_range.contains(&time) {
            time = (-half_b + discriminant_sqrt) / a;
            if !time_range.contains(&time) {
                return None;
            }
        }

        let location = by_ray.at(time);
        let outward_normal = (location - self.center) / self.radius;
        Some(Hit {
            time,
            location,
            normal: if outward_normal.dot(by_ray.direction) < 0.0 {
                outward_normal
            } else {
                -outward_normal
            },
        })
    }
}
