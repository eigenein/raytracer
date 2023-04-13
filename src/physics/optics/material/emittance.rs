use schemars::JsonSchema;
use serde::Deserialize;

use crate::physics::optics::material::property::Property;
use crate::physics::optics::spectrum::black_body;
use crate::physics::units::*;

#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum Emittance {
    /// Black body radiation: <https://en.wikipedia.org/wiki/Planck%27s_law>.
    BlackBody {
        temperature: Temperature,
        scale: Bare,
    },
}

impl Property<SpectralRadianceInWavelength> for Emittance {
    fn at(&self, wavelength: Length) -> SpectralRadianceInWavelength {
        match self {
            Self::BlackBody { scale, temperature } => *scale * black_body(*temperature, wavelength),
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
