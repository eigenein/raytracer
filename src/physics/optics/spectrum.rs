use crate::physics::units::*;

/// <https://en.wikipedia.org/wiki/Spectral_line_shape#Lorentzian>
pub const fn lorentzian(
    wavelength: Length,
    maximum_at: Length,
    full_width_at_half_maximum: Length,
) -> Bare {
    let x = (wavelength - maximum_at) / full_width_at_half_maximum * 2.0;
    Bare::from(1.0) / (x * x + 1.0)
}
