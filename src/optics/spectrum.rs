use crate::math::uom::{Bare, Length};

/// <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>
pub fn lorentzian(
    wavelength: Length,
    maximum_at: Length,
    full_width_at_half_maximum: Length,
) -> Bare {
    let x = (wavelength - maximum_at) / full_width_at_half_maximum * 2.0;
    Bare::from(1.0) / (x.powi::<2>() + 1.0)
}
