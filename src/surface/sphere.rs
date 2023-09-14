use std::ops::Range;

use fastrand::Rng;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::{Aabb, Bounded};
use crate::math::hit::*;
use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::physics::optics::material::Material;

#[derive(Deserialize, JsonSchema)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Bounded for Sphere {
    #[inline]
    fn aabb(&self) -> Aabb {
        Aabb {
            min_point: self.center - self.radius,
            max_point: self.center + self.radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>, _rng: &Rng) -> Option<Hit> {
        let oc = by_ray.origin - self.center;
        let a = by_ray.direction.length_squared();
        let c = oc.length_squared() - self.radius * self.radius;
        let half_b = oc.dot(by_ray.direction);
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
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use super::*;
    use crate::math::ray::Ray;

    #[bench]
    fn bench_hit(bencher: &mut Bencher) {
        let sphere = Sphere {
            center: Default::default(),
            radius: 1.0,
            material: Default::default(),
        };
        let ray = Ray::by_two_points(Vec3::ONE, Vec3::ZERO);
        let rng = Rng::new();
        bencher.iter(|| sphere.hit(&ray, &(0.0..f64::INFINITY), &rng));
    }
}
