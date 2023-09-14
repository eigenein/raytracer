use std::ops::{BitOr, Range};

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

/// Axis-aligned boundary box defined by two points:
/// the one with the minimal coordinates, and the other – with the maximal coordinates.
#[derive(Deserialize, JsonSchema, Copy, Clone, Debug)]
#[must_use]
pub struct Aabb {
    #[serde(alias = "min")]
    pub min_point: Vec3,

    #[serde(alias = "max")]
    pub max_point: Vec3,
}

impl Aabb {
    #[inline]
    pub fn size(&self) -> Vec3 {
        self.max_point - self.min_point
    }

    #[inline]
    pub fn center(&self) -> Vec3 {
        self.min_point + self.size() / 2.0
    }

    /// See the original: <https://gamedev.stackexchange.com/a/18459/171067>.
    pub fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>) -> Option<(f64, f64)> {
        if self.min_point.is_infinite() && self.max_point.is_infinite() {
            return Some((distance_range.start, distance_range.end));
        }

        let min_plane_distances = (self.min_point - by_ray.origin) / by_ray.direction;
        let max_plane_distances = (self.max_point - by_ray.origin) / by_ray.direction;

        let distance_max = min_plane_distances
            .max(max_plane_distances)
            .min_element()
            .min(distance_range.end);

        if distance_max < 0.0 {
            // The is intersecting the box, but the whole box is behind us:
            return None;
        }

        let distance_min = min_plane_distances
            .min(max_plane_distances)
            .max_element()
            .max(distance_range.start);

        (distance_min <= distance_max).then_some((distance_min, distance_max))
    }
}

impl BitOr for Aabb {
    type Output = Self;

    /// Union the two AABBs.
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            min_point: self.min_point.min(rhs.min_point),
            max_point: self.max_point.max(rhs.max_point),
        }
    }
}

/// Implement to provide an AABB.
pub trait Bounded {
    /// Get a boundary box for the surface.
    fn aabb(&self) -> Aabb;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ray::Ray;

    #[test]
    fn hit_ok() {
        let ray = Ray::by_two_points(Vec3::ZERO, Vec3::ONE);
        let aabb = Aabb {
            min_point: Vec3::new(2.0, 2.0, 2.0),
            max_point: Vec3::new(3.0, 3.0, 3.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_some());
    }

    #[test]
    fn hit_from_inside_ok() {
        let ray = Ray::by_two_points(Vec3::ZERO, Vec3::ONE);
        let aabb = Aabb {
            min_point: Vec3::splat(-100.0),
            max_point: Vec3::splat(100.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_some());
    }

    #[test]
    fn no_hit_behind() {
        let ray = Ray::by_two_points(Vec3::ONE, Vec3::ZERO);
        let aabb = Aabb {
            min_point: Vec3::new(2.0, 2.0, 2.0),
            max_point: Vec3::new(3.0, 3.0, 3.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_none());
    }

    #[test]
    fn no_hit_parallel() {
        let ray = Ray::by_two_points(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0));
        let aabb = Aabb {
            min_point: Vec3::new(1.0, 1.0, 1.0),
            max_point: Vec3::new(2.0, 2.0, 2.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_none());
    }

    #[test]
    fn hit_infinity() {
        let ray = Ray::by_two_points(Vec3::ZERO, Vec3::ONE);
        let aabb = Aabb {
            min_point: Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            max_point: Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        };
        assert_eq!(aabb.hit(&ray, &(0.0..f64::INFINITY)), Some((0.0, f64::INFINITY)));
    }
}
