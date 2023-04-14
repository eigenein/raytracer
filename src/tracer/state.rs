use crate::physics::optics::material::attenuation::Attenuation;
use crate::physics::optics::material::emittance::Emittance;
use crate::physics::optics::material::property::Property;
use crate::physics::units::*;

pub enum TraceState {
    /// Tracing a single wavelength.
    SingleLine(TraceSingleLine),
}

impl TraceState {
    pub const fn new(wavelength: Length) -> Self {
        Self::SingleLine(TraceSingleLine {
            wavelength,
            total_radiance: Quantity(0.0),
            total_attenuation: Quantity(1.0),
        })
    }

    /// Sample a single wavelength and collapse the state to the single line.
    #[inline]
    pub fn collapse(&mut self) -> &mut TraceSingleLine {
        match self {
            Self::SingleLine(single_line) => single_line,
        }
    }

    #[inline]
    pub fn add_emittance(&mut self, emittance: &Emittance) {
        match self {
            Self::SingleLine(line) => {
                line.add_emittance(emittance.at(line.wavelength));
            }
        }
    }

    #[inline]
    pub fn mul_attenuation(&mut self, attenuation: &Attenuation) {
        match self {
            Self::SingleLine(line) => {
                line.mul_attenuation(attenuation.at(line.wavelength));
            }
        }
    }

    /// Finalize the state and calculate the total radiance.
    #[inline]
    pub const fn into_radiance(self) -> SpectralRadiancePerMeter {
        match self {
            Self::SingleLine(line) => line.total_radiance,
        }
    }
}

pub struct TraceSingleLine {
    pub wavelength: Length,
    pub total_radiance: SpectralRadiancePerMeter,
    pub total_attenuation: Bare,
}

impl TraceSingleLine {
    #[inline]
    pub fn add_emittance(&mut self, emittance: SpectralRadiancePerMeter) {
        self.total_radiance += self.total_attenuation * emittance;
    }

    #[inline]
    pub fn mul_attenuation(&mut self, attenuation: Bare) {
        self.total_attenuation *= attenuation;
    }
}
