pub mod attenuation;
pub mod emittance;
pub mod property;
pub mod reflectance;
pub mod transmittance;

use schemars::JsonSchema;
use serde::Deserialize;

use self::transmittance::Transmittance;
use crate::physics::optics::material::emittance::Emittance;
use crate::physics::optics::material::reflectance::Reflectance;

#[derive(Default, Deserialize, JsonSchema)]
pub struct Material {
    #[serde(default)]
    pub reflectance: Option<Reflectance>, // TODO: make it a vector.

    #[serde(default)]
    pub transmittance: Option<Transmittance>,

    #[serde(default)]
    pub emittance: Option<Emittance>, // TODO: make it a vector.
}
