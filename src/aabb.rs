use glam::DVec3;

use crate::ray::Ray;

/// Axis-aligned boundary box defined by two points:
/// the one with the minimal coordinates, and the other – with the maximal coordinates.
pub struct Aabb {
    pub min_point: DVec3,
    pub max_point: DVec3,
}

impl Aabb {
    /// See the original: <https://gamedev.stackexchange.com/a/18459/171067>.
    #[allow(dead_code)]
    pub fn hit(&self, ray: &Ray) -> bool {
        let min_plane_distances = (self.min_point - ray.origin) / ray.direction;
        let max_plane_distances = (self.max_point - ray.origin) / ray.direction;

        let distance_max = min_plane_distances.max(max_plane_distances).min_element();

        if distance_max < 0.0 {
            // The is intersecting the box, but the whole box is behind us:
            return false;
        }

        let distance_min = min_plane_distances.min(max_plane_distances).max_element();
        distance_min <= distance_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_ok() {
        let ray = Ray::by_two_points(DVec3::ZERO, DVec3::ONE);
        let aabb = Aabb {
            min_point: DVec3::new(2.0, 2.0, 2.0),
            max_point: DVec3::new(3.0, 3.0, 3.0),
        };
        assert!(aabb.hit(&ray));
    }

    #[test]
    fn no_hit_behind() {
        let ray = Ray::by_two_points(DVec3::ONE, DVec3::ZERO);
        let aabb = Aabb {
            min_point: DVec3::new(2.0, 2.0, 2.0),
            max_point: DVec3::new(3.0, 3.0, 3.0),
        };
        assert!(!aabb.hit(&ray));
    }

    #[test]
    fn no_hit_parallel() {
        let ray = Ray::by_two_points(DVec3::ZERO, DVec3::new(1.0, 0.0, 0.0));
        let aabb = Aabb {
            min_point: DVec3::ONE,
            max_point: DVec3::new(2.0, 2.0, 2.0),
        };
        assert!(!aabb.hit(&ray));
    }
}
