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
}

#[derive(Deserialize)]
pub struct Reflection {
    #[serde(default = "Reflection::default_reflection_color")]
    pub color: DVec4,

    #[serde(default)]
    pub fuzz: f64,
}

impl Reflection {
    pub const fn default_reflection_color() -> DVec4 {
        DVec4::ONE
    }
}
