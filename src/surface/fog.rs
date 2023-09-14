use std::ops::Range;

use fastrand::Rng;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::{Aabb, Bounded};
use crate::math::hit::*;
use crate::math::ray::Ray;
use crate::physics::optics::material::Material;

#[derive(Deserialize, JsonSchema)]
pub struct UniformFog {
    /// Axis-aligned boundary box.
    pub aabb: Aabb,

    /// Fog density.
    #[serde(default = "UniformFog::default_density")]
    pub density: f64,

    pub material: Material,
}

impl UniformFog {
    pub const fn default_density() -> f64 {
        1.0
    }
}

impl Bounded for UniformFog {
    #[inline]
    fn aabb(&self) -> Aabb {
        self.aabb
    }
}

impl Hittable for UniformFog {
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>, rng: &Rng) -> Option<Hit> {
        let Some((min_distance, max_distance)) = self.aabb.hit(by_ray, distance_range) else {
            return None;
        };
        assert!(min_distance.is_finite());
        let hit_distance = min_distance - 1.0 / self.density * rng.f64().ln();
        if hit_distance < max_distance {
            let hit = Hit {
                location: by_ray.at(hit_distance),
                normal: -by_ray.direction,
                distance: hit_distance,
                type_: HitType::Refract,
                material: &self.material,
            };
            Some(hit)
        } else {
            None
        }
    }
}
