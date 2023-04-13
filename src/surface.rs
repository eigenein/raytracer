mod fog;
mod sphere;

use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::Aabb;
use crate::physics::optics::hit::{Hit, Hittable};
use crate::physics::optics::ray::Ray;
use crate::surface::fog::UniformFog;
use crate::surface::sphere::Sphere;

/// Surface that is being rendered.
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Surface {
    Sphere(Sphere),
    UniformFog(UniformFog),
}

impl Hittable for Surface {
    fn hit(&self, by_ray: &Ray, distance: &Range<f64>) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(by_ray, distance),
            Self::UniformFog(fog) => fog.hit(by_ray, distance),
        }
    }

    fn aabb(&self) -> Option<Aabb> {
        match self {
            Self::Sphere(sphere) => sphere.aabb(),
            Self::UniformFog(fog) => Some(fog.aabb),
        }
    }
}
