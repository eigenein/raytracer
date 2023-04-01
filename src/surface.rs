use std::ops::Range;

use glam::DVec3;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::aabb::Aabb;
use crate::hit::{Hit, HitType, Hittable};
use crate::material::Material;
use crate::ray::Ray;

/// Surface that is being rendered.
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Surface {
    Sphere(Sphere),
}

impl Hittable for Surface {
    fn hit(&self, by_ray: &Ray, distance: &Range<f64>) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(by_ray, distance),
        }
    }

    fn aabb(&self) -> Option<Aabb> {
        match self {
            Self::Sphere(sphere) => sphere.aabb(),
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct Sphere {
    #[schemars(with = "[f64; 3]")]
    center: DVec3,

    radius: f64,

    material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>) -> Option<Hit> {
        let oc = by_ray.origin - self.center;
        let a = by_ray.direction.length_squared();
        let half_b = oc.dot(by_ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut distance = (-half_b - discriminant_sqrt) / a;
        if !distance_range.contains(&distance) {
            distance = (-half_b + discriminant_sqrt) / a;
            if !distance_range.contains(&distance) {
                return None;
            }
        }

        let location = by_ray.at(distance);
        let outward_normal = (location - self.center) / self.radius;
        let type_ = if outward_normal.dot(by_ray.direction) < 0.0 {
            HitType::Enter
        } else {
            HitType::Leave
        };

        Some(Hit {
            distance,
            location,
            type_,
            normal: match type_ {
                HitType::Enter => outward_normal,
                HitType::Leave => -outward_normal,
            },
            material: &self.material,
        })
    }

    fn aabb(&self) -> Option<Aabb> {
        Some(Aabb {
            min_point: self.center - self.radius,
            max_point: self.center + self.radius,
        })
    }
}
