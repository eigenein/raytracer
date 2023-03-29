use glam::DVec3;

/// Reflect the vector around the normal.
///
/// # Notes
///
/// The normal **must** be normalized.
///
/// Source: <https://math.stackexchange.com/a/13263/22462>.
pub fn reflect(vector: DVec3, normal: DVec3) -> DVec3 {
    vector - normal * (2.0 * vector.dot(normal))
}

pub fn random_unit_vector() -> DVec3 {
    loop {
        let vector =
            DVec3::new(fastrand::f64() - 0.5, fastrand::f64() - 0.5, fastrand::f64() - 0.5);
        if vector.length_squared() <= 0.25 {
            return vector.normalize();
        }
    }
}
