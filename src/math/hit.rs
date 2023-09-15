use std::cmp::Ordering;
use std::ops::Range;

use serde::Deserialize;

use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::physics::optics::material::Material;

/// Hit result.
pub struct Hit<'a> {
    /// Hit point location.
    pub location: Vec3,

    /// Normal at the hit point.
    pub normal: Vec3,

    /// Distance travelled by the ray till the hit point.
    ///
    /// The ray direction **must** be normalized for this to hold.
    ///
    /// TODO: make it length.
    pub distance: f64,

    pub type_: HitType,

    /// Material at the hit point.
    pub material: &'a Material,
}

impl<'a> PartialEq for Hit<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl<'a> PartialOrd for Hit<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

pub trait Hittable<S> {
    /// Check whether the ray hits the surface.
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>, rng: &mut S) -> Option<Hit>;
}

#[derive(Deserialize, PartialEq, Copy, Clone)]
pub enum HitType {
    /// Ray would have entered the body.
    Enter,

    /// Ray would have left the body.
    Leave,
}
