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

#[derive(Deserialize, JsonSchema)]
pub struct Material {
    #[serde(default)]
    pub reflectance: Option<Reflectance>,

    #[serde(default)]
    pub transmittance: Option<Transmittance>,

    #[serde(default)]
    pub emittance: Option<Emittance>,
}

#[allow(clippy::derivable_impls)]
impl const Default for Material {
    fn default() -> Self {
        Self {
            reflectance: None,
            transmittance: None,
            emittance: None,
        }
    }
}
