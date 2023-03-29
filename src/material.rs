use glam::{DVec3, DVec4};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Material {
    #[serde(default)]
    pub reflection: Option<Reflection>,

    #[serde(default)]
    pub diffusion_color: Option<DVec4>,

    #[serde(default)]
    pub luminance: Option<DVec3>,

    #[serde(default)]
    pub refraction: Option<Refraction>,
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
    #[serde(default = "Refraction::default_color")]
    pub color: DVec4,

    #[serde(default = "Refraction::default_index")]
    pub index: f64,
}

impl Refraction {
    pub const fn default_color() -> DVec4 {
        DVec4::ONE
    }

    pub const fn default_index() -> f64 {
        1.0
    }
}
