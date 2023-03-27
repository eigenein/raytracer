pub mod sphere;

use serde::Deserialize;

use crate::body::sphere::Sphere;
use crate::ray::Ray;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum Body {
    Sphere(Sphere),
}

impl Body {
    pub fn hit_by(&self, ray: &Ray) -> bool {
        match self {
            Self::Sphere(sphere) => sphere.hit_by(ray),
        }
    }
}
