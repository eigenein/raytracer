use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::point::Point;
use crate::physics::optics::ray::Ray;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_ok() {
        let ray = Ray::by_two_points(Point::ZERO, Point::ONE);
        let aabb = Aabb {
            min_point: Point::new(2.0, 2.0, 2.0),
            max_point: Point::new(3.0, 3.0, 3.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_some());
    }

    #[test]
    fn no_hit_behind() {
        let ray = Ray::by_two_points(Point::ONE, Point::ZERO);
        let aabb = Aabb {
            min_point: Point::new(2.0, 2.0, 2.0),
            max_point: Point::new(3.0, 3.0, 3.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_none());
    }

    #[test]
    fn no_hit_parallel() {
        let ray = Ray::by_two_points(Point::ZERO, Point::new(1.0, 0.0, 0.0));
        let aabb = Aabb {
            min_point: Point::new(1.0, 1.0, 1.0),
            max_point: Point::new(2.0, 2.0, 2.0),
        };
        assert!(aabb.hit(&ray, &(0.0..f64::INFINITY)).is_none());
    }

    #[test]
    fn hit_infinity() {
        let ray = Ray::by_two_points(Point::ZERO, Point::ONE);
        let aabb = Aabb {
            min_point: Point::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY),
            max_point: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        };
        assert_eq!(aabb.hit(&ray, &(0.0..f64::INFINITY)), Some((0.0, f64::INFINITY)));
    }
}
