pub mod bare;
pub mod quantity;

pub use self::bare::Bare;
pub use self::quantity::Quantity;

pub type Length<V = f64> = Quantity<V, 0, 1, 0, 0, 0, 0, 0>;

/// Reciprocal length: <https://en.wikipedia.org/wiki/Reciprocal_length>.
pub type ReciprocalLength<V = f64> = Quantity<V, 0, -1, 0, 0, 0, 0, 0>;

pub type Temperature<V = f64> = Quantity<V, 0, 0, 0, 0, 1, 0, 0>;

/// Spectral radiance per steradian per unit wavelength:
/// <https://en.wikipedia.org/wiki/Spectral_radiance>.
pub type SpectralRadiancePerMeter<V = f64> = Quantity<V, -3, -1, 1, 0, 0, 0, 0>;
