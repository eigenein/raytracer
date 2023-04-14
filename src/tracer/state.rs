use crate::physics::optics::material::attenuation::Attenuation;
use crate::physics::optics::material::emittance::Emittance;
use crate::physics::optics::material::property::Property;
use crate::physics::units::*;

pub enum TraceState<'a> {
    /// Tracing the single wavelength.
    SingleLine(SingleLine),

    /// Tracing the entire spectrum.
    Spectrum(Spectrum<'a>),
}

impl<'a> TraceState<'a> {
    /// Sample a single wavelength and collapse the state to the single line.
    #[inline]
    pub fn collapse(&mut self) -> &mut SingleLine {
        match self {
            Self::SingleLine(line) => line,
            Self::Spectrum(spectrum) => {
                *self = Self::SingleLine(spectrum.collapse());
                match self {
                    Self::SingleLine(line) => line,
                    _ => unreachable!(),
                }
            }
        }
    }

    #[inline]
    pub fn apply_emittance<'b: 'a>(&mut self, emittance: &'b Emittance) {
        match self {
            Self::SingleLine(line) => {
                line.apply_emittance(emittance.at(line.wavelength));
            }
            Self::Spectrum(spectrum) => spectrum.apply_emittance(emittance),
        }
    }

    #[inline]
    pub fn apply_attenuation<'b: 'a>(&mut self, attenuation: &'b Attenuation) {
        match self {
            Self::SingleLine(line) => {
                line.apply_attenuation(attenuation.at(line.wavelength));
            }
            Self::Spectrum(spectrum) => spectrum.apply_attenuation(attenuation),
        }
    }

    /// Finalize the state and calculate the total radiance.
    #[inline]
    pub fn into_radiance(self) -> (Length, SpectralRadiancePerMeter) {
        let line = match self {
            Self::SingleLine(line) => line,
            Self::Spectrum(spectrum) => spectrum.collapse(),
        };
        (line.wavelength, line.total_radiance)
    }
}

pub struct SingleLine {
    pub wavelength: Length,

    total_radiance: SpectralRadiancePerMeter,
    total_attenuation: Bare,
}

impl SingleLine {
    #[inline]
    pub fn apply_emittance(&mut self, emittance: SpectralRadiancePerMeter) {
        self.total_radiance += self.total_attenuation * emittance;
    }

    #[inline]
    pub fn apply_attenuation(&mut self, attenuation: Bare) {
        self.total_attenuation *= attenuation;
    }
}

pub struct Spectrum<'a> {
    min_wavelength: Length,
    max_wavelength: Length,
    emitters: Vec<(usize, &'a Emittance)>,
    attenuators: Vec<&'a Attenuation>,
}

impl<'a> Spectrum<'a> {
    #[inline]
    pub const fn new(min_wavelength: Length, max_wavelength: Length) -> Self {
        Self {
            min_wavelength,
            max_wavelength,
            emitters: Vec::new(),
            attenuators: Vec::new(),
        }
    }

    #[inline]
    pub fn apply_emittance(&mut self, emittance: &'a Emittance) {
        self.emitters.push((self.attenuators.len(), emittance));
    }

    #[inline]
    pub fn apply_attenuation(&mut self, attenuation: &'a Attenuation) {
        self.attenuators.push(attenuation);
    }

    #[inline]
    pub fn radiance_at(&self, wavelength: Length) -> SpectralRadiancePerMeter {
        self.emitters
            .iter()
            .map(|(n_attenuators, emittance)| {
                emittance.at(wavelength) * self.total_attenuation(wavelength, *n_attenuators)
            })
            .sum()
    }

    #[inline]
    pub fn total_attenuation(&self, wavelength: Length, n_attenuators: usize) -> Bare {
        self.attenuators[0..n_attenuators]
            .iter()
            .fold(Quantity(1.0), |attenuation, attenuator| attenuation * attenuator.at(wavelength))
    }

    #[inline]
    pub fn collapse(&self) -> SingleLine {
        let wavelength = self.min_wavelength
            + Bare::from(fastrand::f64()) * (self.max_wavelength - self.min_wavelength);
        SingleLine {
            wavelength,
            total_attenuation: self.total_attenuation(wavelength, self.attenuators.len()),
            total_radiance: self.radiance_at(wavelength),
        }
    }
}
