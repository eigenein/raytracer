use schemars::JsonSchema;
use serde::Deserialize;

use crate::lighting::spectrum::Spectrum;

#[derive(Deserialize, JsonSchema)]
pub struct Material {
    #[serde(default)]
    pub reflectance: Option<Reflectance>,

    #[serde(default)]
    pub transmittance: Option<Transmittance>,

    #[serde(default)]
    pub emittance: Option<Spectrum>,
}

#[derive(Deserialize, JsonSchema, Default)]
pub struct Reflectance {
    /// Absorbs nothing anything by default.
    #[serde(default)]
    pub attenuation: Spectrum,

    #[serde(default)]
    pub fuzz: Option<f64>,

    #[serde(default, alias = "diffuse")]
    pub diffusion: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
pub struct Transmittance {
    /// Refractive index of the medium inside the body.
    /// By default, this is the index of vacuum.
    #[serde(default = "Transmittance::default_refractive_index", alias = "index")]
    pub refractive_index: f64,

    #[serde(default)]
    pub attenuation: Spectrum,

    /// Attenuation coefficient: <https://en.wikipedia.org/wiki/Attenuation_coefficient>.
    /// Considered to be zero by default.
    #[serde(default)]
    pub coefficient: Option<f64>,
}

impl Transmittance {
    pub const fn default_refractive_index() -> f64 {
        1.0
    }
}
