use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum Attenuation {
    Constant {
        coefficient: f64,
    },

    /// Empirical approximation based on
    /// <https://en.wikipedia.org/wiki/Electromagnetic_absorption_by_water#/media/File:Absorption_coefficient_of_water.svg>.
    Water {
        scale: f64,
    },
}

impl Attenuation {
    pub fn at(self, wavelength: f64) -> f64 {
        match self {
            Self::Constant { coefficient } => coefficient,

            Self::Water { scale } => scale * 10.0_f64.powf((wavelength - 450e-9) * 0.75e7),
        }
    }
}
