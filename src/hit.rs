use glam::DVec3;

use crate::material::Material;

/// Hit result.
pub struct Hit<'a> {
    /// Hit point location.
    pub location: DVec3,

    /// Normal at the hit point.
    pub normal: DVec3,

    /// Time travelled by the ray till the hit point.
    pub time: f64,

    pub from_outside: bool,

    /// Material at the hit point.
    pub material: &'a Material,
}
