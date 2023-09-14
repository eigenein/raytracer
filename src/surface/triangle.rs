use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::{Aabb, Bounded};
use crate::math::hit::{Hit, HitType, Hittable};
use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::physics::optics::material::Material;

#[derive(Deserialize, JsonSchema)]
pub struct Triangle {
    vertices: [Vec3; 3],
    material: Material,
}

impl Bounded for Triangle {
    fn aabb(&self) -> Aabb {
        Aabb {
            min_point: self.vertices[0].min(self.vertices[1].min(self.vertices[2])),
            max_point: self.vertices[0].max(self.vertices[1].max(self.vertices[2])),
        }
    }
}

impl<S> Hittable<S> for Triangle {
    /// Implement [Möller–Trumbore intersection algorithm][1].
    /// I have no idea what's going on here.
    ///
    /// [1]: https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>, _rng: &mut S) -> Option<Hit> {
        let edge_1 = self.vertices[1] - self.vertices[0];
        let edge_2 = self.vertices[2] - self.vertices[0];

        let h = by_ray.direction.cross(edge_2);
        let a = edge_1.dot(h);

        let f = 1.0 / a;
        let s = by_ray.origin - self.vertices[0];
        let u = f * s.dot(h);

        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let q = s.cross(edge_1);
        let v = f * by_ray.direction.dot(q);

        if v < 0.0 || (u + v > 1.0) {
            return None;
        }

        let distance = f * edge_2.dot(q);
        if distance_range.contains(&distance) {
            let mut normal = edge_1.cross(edge_2).normalize();
            if normal.dot(by_ray.direction) > 0.0 {
                normal = -normal;
            }

            Some(Hit {
                location: by_ray.at(distance),
                normal,
                distance,
                type_: HitType::Refract,
                material: &self.material,
            })
        } else {
            None
        }
    }
}
