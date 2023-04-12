pub mod emittance;
pub mod property;
pub mod reflectance;
pub mod transmittance;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::optics::material::reflectance::Spectrum;
use crate::optics::material::transmittance::refraction::RefractiveIndex;
use crate::optics::material::transmittance::TransmissionAttenuation;

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
    #[serde(default)]
    pub attenuation: Spectrum,

    #[serde(default)]
    pub fuzz: Option<f64>, // TODO: this may relate to transmittance as well.

    #[serde(default, alias = "diffuse")]
    pub diffusion: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
pub struct Transmittance {
    /// Refractive index of the medium inside the body.
    /// By default, this is the index of vacuum.
    #[serde(default, alias = "index")]
    pub refractive_index: RefractiveIndex,

    /// Attenuation of the body inner material.
    #[serde(default)]
    pub attenuation: Spectrum,

    /// Attenuation coefficient: <https://en.wikipedia.org/wiki/Attenuation_coefficient>.
    /// Considered to be zero by default.
    #[serde(default)]
    pub coefficient: Option<TransmissionAttenuation>,
}
