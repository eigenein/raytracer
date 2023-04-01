use glam::DVec3;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct Material {
    #[serde(default)]
    pub reflectance: Reflectance,

    #[serde(default)]
    pub transmittance: Option<Transmittance>,

    #[serde(default)]
    #[schemars(with = "[f64; 3]")]
    pub emittance: DVec3,
}

#[derive(Deserialize, JsonSchema)]
pub struct Reflectance {
    #[serde(default = "Reflectance::default_attenuation")]
    #[schemars(with = "[f64; 3]")]
    pub attenuation: DVec3,

    #[serde(default)]
    pub fuzz: Option<f64>,

    #[serde(default, alias = "diffuse")]
    pub diffusion: Option<f64>,
}

impl Reflectance {
    pub const fn default_attenuation() -> DVec3 {
        DVec3::ONE
    }
}

impl Default for Reflectance {
    fn default() -> Self {
        Self {
            attenuation: Self::default_attenuation(),
            fuzz: None,
            diffusion: None,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct Transmittance {
    /// Refractive index of the medium inside the body.
    #[serde(default = "Transmittance::default_refractive_index", alias = "index")]
    pub refractive_index: f64,

    /// If not set, defaults to the reflectance attenuation.
    #[serde(default)]
    #[schemars(with = "Option<[f64; 3]>")]
    pub attenuation: Option<DVec3>,

    /// Attenuation coefficient: <https://en.wikipedia.org/wiki/Attenuation_coefficient>.
    #[serde(default)]
    pub coefficient: Option<f64>,
}

impl Transmittance {
    pub const fn default_refractive_index() -> f64 {
        1.0
    }
}
