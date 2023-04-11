use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::uom::{AttenuationCoefficient, Bare, Length};

#[derive(Copy, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Attenuation {
    Constant {
        coefficient: AttenuationCoefficient,
    },

    /// Empirical approximation based on
    /// <https://en.wikipedia.org/wiki/Electromagnetic_absorption_by_water#/media/File:Absorption_coefficient_of_water.svg>.
    Water {
        scale: AttenuationCoefficient,
    },
}

impl Attenuation {
    pub fn at(self, wavelength: Length) -> AttenuationCoefficient {
        match self {
            Self::Constant { coefficient } => coefficient,

            Self::Water { scale } => {
                scale
                    * Bare::from(10.0_f64).powf(
                        (wavelength - Length::from_nanos(450.0))
                            * AttenuationCoefficient::from(0.75e7),
                    )
            }
        }
    }
}
