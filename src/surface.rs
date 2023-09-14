mod fog;
mod sphere;
mod triangle;

use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::{Aabb, Bounded};
use crate::math::hit::*;
use crate::math::ray::Ray;
use crate::math::sequence::Sequence;
use crate::surface::fog::UniformFog;
use crate::surface::sphere::Sphere;
use crate::surface::triangle::Triangle;

/// Surface that is being rendered.
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Surface {
    Sphere(Sphere),
    Triangle(Triangle),
    UniformFog(UniformFog),
}

impl Bounded for Surface {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Sphere(sphere) => sphere.aabb(),
            Self::Triangle(triangle) => triangle.aabb(),
            Self::UniformFog(fog) => fog.aabb,
        }
    }
}

impl<S: Sequence<f64>> Hittable<S> for Surface {
    fn hit(&self, by_ray: &Ray, distance: &Range<f64>, rng: &mut S) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(by_ray, distance, rng),
            Self::Triangle(triangle) => triangle.hit(by_ray, distance, rng),
            Self::UniformFog(fog) => fog.hit(by_ray, distance, rng),
        }
    }
}
