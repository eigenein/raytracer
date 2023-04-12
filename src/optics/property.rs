use crate::math::uom::Length;

/// An optical property.
///
/// I don't really need it in the first place, but it helps to structure the code.
#[const_trait]
pub trait Property<V> {
    /// Get the property value at the given wavelength.
    fn at(&self, wavelength: Length) -> V;
}
