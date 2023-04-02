use schemars::JsonSchema;
use serde::Deserialize;

use crate::lighting::spectrum::Spectrum;

#[derive(Deserialize, JsonSchema)]
pub struct Material {
    #[serde(default)]
    pub reflectance: Reflectance,

    #[serde(default)]
    pub transmittance: Option<Transmittance>,

    #[serde(default)]
    pub emittance: Option<Spectrum>,
}

#[derive(Deserialize, JsonSchema)]
pub struct Reflectance {
    #[serde(default = "Reflectance::default_attenuation")]
    pub attenuation: Spectrum,

    #[serde(default)]
    pub fuzz: Option<f64>,

    #[serde(default, alias = "diffuse")]
    pub diffusion: Option<f64>,
}

impl const Default for Reflectance {
    fn default() -> Self {
        Self {
            attenuation: Self::default_attenuation(),
            fuzz: None,
            diffusion: None,
        }
    }
}

impl Reflectance {
    pub const fn default_attenuation() -> Spectrum {
        Spectrum::Constant { intensity: 1.0 }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct Transmittance {
    /// Refractive index of the medium inside the body.
    #[serde(default = "Transmittance::default_refractive_index", alias = "index")]
    pub refractive_index: f64,

    /// If not set, defaults to the reflectance attenuation.
    #[serde(default)]
    pub attenuation: Option<Spectrum>,

    /// Attenuation coefficient: <https://en.wikipedia.org/wiki/Attenuation_coefficient>.
    #[serde(default)]
    pub coefficient: Option<f64>,
}

impl Transmittance {
    pub const fn default_refractive_index() -> f64 {
        1.0
    }
}
