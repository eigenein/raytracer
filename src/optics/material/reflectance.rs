use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::uom::{Bare, Length, Temperature};
use crate::optics::material::property::Property;
use crate::optics::spectrum::{black_body, lorentzian};

/// Absorbs nothing by default.
#[derive(Deserialize, JsonSchema, Clone)]
#[serde(tag = "type")]
pub enum ReflectanceAttenuation {
    Constant {
        #[serde(default = "ReflectanceAttenuation::default_intensity")]
        intensity: Bare,
    },

    /// <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>
    Lorentzian {
        #[serde(
            default = "ReflectanceAttenuation::default_intensity",
            alias = "intensity"
        )]
        max_intensity: Bare,

        /// Wavelength of the maximum, meters.
        #[serde(alias = "max", alias = "maximum")]
        maximum_at: Length,

        /// <https://en.wikipedia.org/wiki/Full_width_at_half_maximum>
        #[serde(alias = "fwhm")]
        full_width_at_half_maximum: Length,
    },

    /// Black body radiation: <https://en.wikipedia.org/wiki/Planck%27s_law>.
    ///
    /// TODO: extract into `Emittance`.
    BlackBody {
        temperature: Temperature,
        scale: Bare,
    },

    /// Sum of the spectra.
    Sum {
        spectra: Vec<ReflectanceAttenuation>,
    },
}

impl const Default for ReflectanceAttenuation {
    fn default() -> Self {
        ReflectanceAttenuation::Constant { intensity: Bare::from(1.0) }
    }
}

impl ReflectanceAttenuation {
    pub const fn default_intensity() -> Bare {
        Bare::from(1.0)
    }
}

impl Property<Bare> for ReflectanceAttenuation {
    fn at(&self, wavelength: Length) -> Bare {
        match self {
            Self::Constant { intensity } => *intensity,

            Self::Lorentzian {
                max_intensity,
                maximum_at,
                full_width_at_half_maximum,
            } => *max_intensity * lorentzian(wavelength, *maximum_at, *full_width_at_half_maximum),

            Self::BlackBody { scale, temperature } => {
                let spectral_radiance = *scale * black_body(*temperature, wavelength);
                // FIXME: the units.
                Bare::from(f64::from(spectral_radiance))
            }

            Self::Sum { spectra } => spectra
                .iter()
                .map(|attenuation| attenuation.at(wavelength))
                .sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lorentzian_ok() {
        let maximum_at = Length::from_nanos(450.0); // blue
        let fwhm = Length::from(1e-14);
        let spectrum = ReflectanceAttenuation::Lorentzian {
            max_intensity: Bare::from(1.0),
            maximum_at,
            full_width_at_half_maximum: fwhm,
        };

        let intensity_at_half_width = spectrum.at(maximum_at - fwhm / Bare::from(2.0));
        assert!(
            (intensity_at_half_width - Bare::from(0.5)).abs() < Bare::from(1e-8),
            "actual: {intensity_at_half_width}"
        );
    }

    #[test]
    fn black_body_ok() {
        let spectrum = ReflectanceAttenuation::BlackBody {
            temperature: Temperature::from(5777.0),
            scale: Bare::from(1.0),
        };
        let intensity = spectrum.at(Length::from_nanos(500.0));
        assert!(
            (intensity - Bare::from(2.635e13)).abs() < Bare::from(1e10),
            "actual: {intensity}"
        );
    }
}
