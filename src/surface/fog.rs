use std::ops::Range;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::aabb::Aabb;
use crate::physics::optics::hit::{Hit, HitType, Hittable};
use crate::physics::optics::material::Material;
use crate::physics::optics::ray::Ray;

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

impl Hittable for UniformFog {
    fn hit(&self, by_ray: &Ray, distance_range: &Range<f64>) -> Option<Hit> {
        let Some((min_distance, max_distance)) = self.aabb.hit(by_ray, distance_range) else { return None };
        assert!(min_distance.is_finite());
        let hit_distance = min_distance - 1.0 / self.density * fastrand::f64().ln();
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

    fn aabb(&self) -> Option<Aabb> {
        Some(self.aabb)
    }
}
