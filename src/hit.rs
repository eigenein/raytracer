use std::ops::Range;

use glam::DVec3;
use serde::Deserialize;

use crate::aabb::Aabb;
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

    pub type_: HitType,

    /// Material at the hit point.
    pub material: &'a Material,
}

pub trait Hittable {
    /// Check whether the ray hits the surface.
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>) -> Option<Hit>;

    /// Get a boundary box for the surface.
    fn aabb(&self) -> Option<Aabb>;
}

#[derive(Deserialize, PartialEq, Copy, Clone)]
pub enum HitType {
    /// Ray would have entered the body.
    Enter,

    /// Ray would have left the body.
    Leave,
}
