use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::consts::*;
use crate::physics::optics::material::property::Property;
use crate::physics::optics::spectrum::lorentzian;
use crate::physics::units::*;

#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum Emittance {
    Constant {
        density: SpectralFluxDensity,
    },

    /// Black body radiation: <https://en.wikipedia.org/wiki/Planck%27s_law>.
    BlackBody {
        temperature: Temperature,
    },

    /// Lorentzian line: <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>.
    Lorentzian {
        maximum: SpectralFluxDensity,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: Length,

        /// <https://en.wikipedia.org/wiki/Full_width_at_half_maximum>
        #[serde(alias = "fwhm")]
        full_width_at_half_maximum: Length,
    },
}

impl Default for Emittance {
    fn default() -> Self {
        Self::Constant { density: Quantity::ZERO }
    }
}

impl Property<SpectralFluxDensity> for Emittance {
    fn at(&self, wavelength: Length) -> SpectralFluxDensity {
        match self {
            Self::Constant { density } => *density,

            Self::BlackBody { temperature } => {
                Bare::from(2.0) * PLANCK * LIGHT_SPEED.squared()
                    / wavelength.quintic()
                    / ((PLANCK * LIGHT_SPEED / wavelength / BOLTZMANN / *temperature).exp() - 1.0)
            }

            Self::Lorentzian {
                maximum,
                maximum_at,
                full_width_at_half_maximum,
            } => *maximum * lorentzian(wavelength, *maximum_at, *full_width_at_half_maximum),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_body_ok() {
        let spectrum = Emittance::BlackBody {
            temperature: Temperature::from(5777.0),
        };
        let intensity = spectrum.at(Length::from_nanos(500.0));
        assert!(
            (intensity - Quantity::from(2.635e13)).abs() < Quantity::from(1e10),
            "actual: {intensity}"
        );
    }
}
