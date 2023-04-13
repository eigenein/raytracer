use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::optics::material::attenuation::Attenuation;

#[derive(Deserialize, JsonSchema, Default)]
pub struct Reflectance {
    #[serde(default)]
    pub attenuation: Attenuation,

    #[serde(default)]
    pub fuzz: Option<f64>, // TODO: this may relate to transmittance as well.

    #[serde(default, alias = "diffuse")]
    pub diffusion: Option<f64>,
}
