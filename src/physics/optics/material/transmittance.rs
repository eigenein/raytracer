pub mod refraction;

use schemars::JsonSchema;
use serde::Deserialize;

use self::refraction::AbsoluteRefractiveIndex;
use crate::physics::optics::material::property::Property;
use crate::physics::units::*;

#[derive(Deserialize, JsonSchema)]
pub struct Transmittance {
    /// Refractive index of the medium **inside** the body.
    #[serde(default, alias = "refracted")]
    pub refracted_index: AbsoluteRefractiveIndex,

    /// Refractive index of the medium **outside** the body.
    ///
    /// By default, this is the index of vacuum.
    #[serde(default, alias = "incident")]
    pub incident_index: AbsoluteRefractiveIndex,

    /// [Attenuation coefficient][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Attenuation_coefficient
    #[serde(alias = "attenuation")]
    pub attenuation_coefficient: AttenuationCoefficient,
}

#[derive(Copy, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
/// TODO: needs more options, including colored material.
pub enum AttenuationCoefficient {
    Constant {
        coefficient: ReciprocalLength,
    },

    /// Empirical approximation based on
    /// <https://en.wikipedia.org/wiki/Electromagnetic_absorption_by_water#/media/File:Absorption_coefficient_of_water.svg>.
    Water {
        scale: ReciprocalLength,
    },
}

impl Property<ReciprocalLength> for AttenuationCoefficient {
    fn at(&self, wavelength: Length) -> ReciprocalLength {
        match self {
            Self::Constant { coefficient } => *coefficient,

            Self::Water { scale } => {
                // FIXME: find a better model.
                *scale
                    * Bare::from(10.0_f64)
                        .powf((wavelength - Length::from_nanos(450.0)) / Length::from_nanos(133.3))
            }
        }
    }
}
