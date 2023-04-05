use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::point::Point;
use crate::ray::Ray;

/// Axis-aligned boundary box defined by two points:
/// the one with the minimal coordinates, and the other – with the maximal coordinates.
#[derive(Deserialize, JsonSchema, Copy, Clone)]
pub struct Aabb {
    #[serde(alias = "min")]
    pub min_point: Point,

    #[serde(alias = "max")]
    pub max_point: Point,
}

impl Aabb {
    /// See the original: <https://gamedev.stackexchange.com/a/18459/171067>.
    #[allow(dead_code)]
    pub fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>) -> Option<(f64, f64)> {
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

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use super::*;

    #[test]
    fn hit_ok() {
        let ray = Ray::by_two_points(DVec3::ZERO.into(), DVec3::ONE.into());
        let aabb = Aabb {
            min_point: DVec3::new(2.0, 2.0, 2.0).into(),
            max_point: DVec3::new(3.0, 3.0, 3.0).into(),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_some());
    }

    #[test]
    fn no_hit_behind() {
        let ray = Ray::by_two_points(DVec3::ONE.into(), DVec3::ZERO.into());
        let aabb = Aabb {
            min_point: DVec3::new(2.0, 2.0, 2.0).into(),
            max_point: DVec3::new(3.0, 3.0, 3.0).into(),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_none());
    }

    #[test]
    fn no_hit_parallel() {
        let ray = Ray::by_two_points(DVec3::ZERO.into(), DVec3::new(1.0, 0.0, 0.0).into());
        let aabb = Aabb {
            min_point: DVec3::new(1.0, 1.0, 1.0).into(),
            max_point: DVec3::new(2.0, 2.0, 2.0).into(),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_none());
    }
}
