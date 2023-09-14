pub mod bare;
pub mod quantity;

pub use self::bare::Bare;
pub use self::quantity::Quantity;

pub type Length = Quantity<0, 1, 0, 0, 0, 0, 0>;

pub type Velocity = Quantity<-1, 1, 0, 0, 0, 0, 0>;

/// [Reciprocal length][1].
///
/// [1]: https://en.wikipedia.org/wiki/Reciprocal_length
pub type ReciprocalLength = Quantity<0, -1, 0, 0, 0, 0, 0>;

pub type Temperature = Quantity<0, 0, 0, 0, 1, 0, 0>;

/// [Spectral radiance][1] per steradian per unit wavelength.
///
/// [1]: https://en.wikipedia.org/wiki/Spectral_radiance
pub type SpectralRadiancePerMeter = Quantity<-3, -1, 1, 0, 0, 0, 0>;
