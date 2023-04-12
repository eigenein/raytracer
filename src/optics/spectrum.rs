use crate::math::uom::*;
use crate::optics::consts::{BOLTZMANN, LIGHT_SPEED, PLANCK};

/// <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>
pub fn lorentzian(
    wavelength: Length,
    maximum_at: Length,
    full_width_at_half_maximum: Length,
) -> Bare {
    let x = (wavelength - maximum_at) / full_width_at_half_maximum * 2.0;
    Bare::from(1.0) / (x.powi::<2>() + 1.0)
}

/// Black body radiation: <https://en.wikipedia.org/wiki/Planck%27s_law>.
pub fn black_body(temperature: Temperature, at_wavelength: Length) -> SpectralRadianceInWavelength {
    Bare::from(2.0) * PLANCK * LIGHT_SPEED.powi::<2>()
        / at_wavelength.powi::<5>()
        / ((PLANCK * LIGHT_SPEED / at_wavelength / BOLTZMANN / temperature).exp() - 1.0)
}
