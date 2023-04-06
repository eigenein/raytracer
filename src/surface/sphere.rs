use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::aabb::Aabb;
use crate::hit::{Hit, HitType, Hittable};
use crate::material::Material;
use crate::math::point::Point;
use crate::physics::ray::Ray;

#[derive(Deserialize, JsonSchema)]
pub struct Sphere {
    center: Point,
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
        let (type_, normal) = if outward_normal.dot(by_ray.direction) < 0.0 {
            (HitType::Enter, outward_normal)
        } else {
            (HitType::Leave, -outward_normal)
        };

        Some(Hit {
            distance,
            location,
            type_,
            normal,
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
