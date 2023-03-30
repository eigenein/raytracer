use glam::{DVec3, DVec4};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Material {
    #[serde(default = "Material::default_attenuation")]
    pub attenuation: DVec3,

    #[serde(default = "Material::default_albedo")]
    pub albedo: f64,

    #[serde(default)]
    pub refractive_index: Option<f64>,

    #[serde(default)]
    pub reflective_fuzz: Option<f64>,

    #[serde(default, alias = "diffusion")]
    pub diffusion_fraction: Option<f64>,
}

impl Material {
    pub const fn default_attenuation() -> DVec3 {
        DVec3::ONE
    }

    pub const fn default_albedo() -> f64 {
        1.0
    }
}

#[derive(Deserialize)]
pub struct Reflection {
    #[serde(default = "Reflection::default_color")]
    pub color: DVec4,

    #[serde(default)]
    pub fuzz: f64,
}

impl Reflection {
    pub const fn default_color() -> DVec4 {
        DVec4::ONE
    }
}

#[derive(Deserialize)]
pub struct Refraction {
    #[serde(default = "Refraction::default_index")]
    pub index: f64,
}

impl Refraction {
    pub const fn default_index() -> f64 {
        1.0
    }
}
