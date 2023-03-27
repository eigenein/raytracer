use glam::DVec3;

/// Hit result.
pub struct Hit {
    /// Hit point location.
    pub location: DVec3,

    /// Normal at the hit point.
    pub normal: DVec3,

    /// Time travelled by the ray till the hit point.
    pub time: f64,
}
