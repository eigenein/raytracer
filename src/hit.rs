use std::ops::Range;

use glam::DVec3;

use crate::material::Material;
use crate::ray::Ray;

/// Hit result.
pub struct Hit<'a> {
    /// Hit point location.
    pub location: DVec3,

    /// Normal at the hit point.
    pub normal: DVec3,

    /// Distance travelled by the ray till the hit point.
    ///
    /// The ray direction **must** be normalized for this to hold.
    pub distance: f64,

    pub from_outside: bool,

    /// Material at the hit point.
    pub material: &'a Material,
}

pub trait Hittable {
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>) -> Option<Hit>;
}
