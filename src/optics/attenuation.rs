use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::uom::{Bare, Length, ReciprocalLength};
use crate::optics::property::Property;

#[derive(Copy, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Attenuation {
    Constant {
        coefficient: ReciprocalLength,
    },

    /// Empirical approximation based on
    /// <https://en.wikipedia.org/wiki/Electromagnetic_absorption_by_water#/media/File:Absorption_coefficient_of_water.svg>.
    Water {
        scale: ReciprocalLength,
    },
}

impl Property<ReciprocalLength> for Attenuation {
    fn at(&self, wavelength: Length) -> ReciprocalLength {
        match self {
            Self::Constant { coefficient } => *coefficient,

            Self::Water { scale } => {
                *scale
                    * Bare::from(10.0_f64).powf(
                        (wavelength - Length::from_nanos(450.0)) * ReciprocalLength::from(0.75e7),
                    )
            }
        }
    }
}
