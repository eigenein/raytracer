pub mod bare;
pub mod quantity;

pub use self::bare::Bare;
pub use self::quantity::Quantity;

#[allow(dead_code)]
pub type Time = Quantity<1, 0, 0, 0, 0>;

#[allow(dead_code)]
pub type Length = Quantity<0, 1, 0, 0, 0>;

#[allow(dead_code)]
pub type Mass = Quantity<0, 0, 1, 0, 0>;

#[allow(dead_code)]
pub type Temperature = Quantity<0, 0, 0, 1, 0>;

/// [Solid angle][1].
///
/// [1]: https://en.wikipedia.org/wiki/Solid_angle
#[allow(dead_code)]
pub type SolidAngle = Quantity<0, 0, 0, 0, 1>;

/// [Reciprocal length][1].
///
/// [1]: https://en.wikipedia.org/wiki/Reciprocal_length
#[allow(dead_code)]
pub type ReciprocalLength = Quantity<0, -1, 0, 0, 0>;

#[allow(dead_code)]
pub type Velocity = Quantity<-1, 1, 0, 0, 0>;

/// [Energy][1].
///
/// [1]: https://en.m.wikipedia.org/wiki/Energy
#[allow(dead_code)]
pub type Energy = Quantity<-2, 2, 1, 0, 0>;

/// [Radiant flux][1] – the radiant energy emitted, reflected, transmitted, or received per unit time.
///
/// [1]: https://en.wikipedia.org/wiki/Radiant_flux
#[allow(dead_code)]
pub type RadiantFlux = Quantity<-3, 2, 1, 0, 0>;

/// Spectral flux per unit wavelength.
#[allow(dead_code)]
pub type SpectralFlux = Quantity<-3, 1, 1, 0, 0>;

/// [Spectral radiance][1] per unit wavelength.
///
/// [1]: https://en.wikipedia.org/wiki/Spectral_radiance
#[allow(dead_code)]
pub type SpectralRadiance = Quantity<-3, -1, 1, 0, -1>;
