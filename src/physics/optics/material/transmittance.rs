pub mod refraction;

use schemars::JsonSchema;
use serde::Deserialize;

use self::refraction::AbsoluteRefractiveIndex;
use crate::physics::optics::material::attenuation::Attenuation;
use crate::physics::optics::material::property::Property;
use crate::physics::units::*;

#[derive(Deserialize, JsonSchema)]
pub struct Transmittance {
    /// Refractive index of the medium **inside** the body.
    pub refracted_index: AbsoluteRefractiveIndex,

    /// Refractive index of the medium **outside** the body.
    ///
    /// By default, this is the index of vacuum.
    #[serde(default)]
    pub incident_index: AbsoluteRefractiveIndex,

    /// Attenuation of the body inner material.
    ///
    /// FIXME: remove.
    #[serde(default)]
    pub attenuation: Attenuation,

    /// [Attenuation coefficient][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Attenuation_coefficient
    ///
    /// FIXME: rename to `attenuation_coefficient`, alias to `attenuation`.
    #[serde(default)]
    pub coefficient: Option<AttenuationCoefficient>,
}

#[derive(Copy, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
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
