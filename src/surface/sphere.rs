use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::Aabb;
use crate::math::point::Point;
use crate::physics::optics::hit::{Hit, HitType, Hittable};
use crate::physics::optics::material::Material;
use crate::physics::optics::ray::Ray;

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

    #[inline]
    fn aabb(&self) -> Option<Aabb> {
        Some(Aabb {
            min_point: self.center - self.radius,
            max_point: self.center + self.radius,
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_hit(bencher: &mut Bencher) {
        let sphere = Sphere {
            center: Default::default(),
            radius: 1.0,
            material: Default::default(),
        };
        let ray = Ray::by_two_points(Point::ONE, Point::ZERO);
        bencher.iter(|| sphere.hit(&ray, &(0.0..f64::INFINITY)));
    }
}
