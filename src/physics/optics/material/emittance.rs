use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::optics::material::property::Property;
use crate::physics::optics::spectrum::{black_body, lorentzian};
use crate::physics::units::*;

#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum Emittance {
    /// Black body radiation: <https://en.wikipedia.org/wiki/Planck%27s_law>.
    BlackBody {
        temperature: Temperature,
        scale: Bare,
    },

    /// Lorentzian line: <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>.
    Lorentzian {
        radiance: SpectralRadiancePerMeter,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: Length,

        /// <https://en.wikipedia.org/wiki/Full_width_at_half_maximum>
        #[serde(alias = "fwhm")]
        full_width_at_half_maximum: Length,
    },
}

impl Property<SpectralRadiancePerMeter> for Emittance {
    fn at(&self, wavelength: Length) -> SpectralRadiancePerMeter {
        match self {
            Self::BlackBody { scale, temperature } => *scale * black_body(*temperature, wavelength),

            Self::Lorentzian {
                radiance,
                maximum_at,
                full_width_at_half_maximum,
            } => *radiance * lorentzian(wavelength, *maximum_at, *full_width_at_half_maximum),
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
            scale: Bare::from(1.0),
        };
        let intensity = spectrum.at(Length::from_nanos(500.0));
        assert!(
            (intensity - Quantity::from(2.635e13)).abs() < Quantity::from(1e10),
            "actual: {intensity}"
        );
    }
}
