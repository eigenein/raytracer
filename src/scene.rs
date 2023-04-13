use std::fs;
use std::path::PathBuf;

use glam::DVec3;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::point::Point;
use crate::physics::optics::material::reflectance::ReflectanceAttenuation;
use crate::physics::units::Bare;
use crate::prelude::*;
use crate::surface::Surface;

/// A scene to render.
///
/// This is a root object in a scene TOML file.
#[derive(Deserialize, JsonSchema)]
pub struct Scene {
    #[serde(default)]
    pub camera: Camera,

    /// Scene background and ambient color.
    #[serde(alias = "ambient_spectrum")]
    pub ambient_emittance: ReflectanceAttenuation,

    /// Scene medium refractive index.
    ///
    /// This index is assigned for the primary incident rays originating
    /// from the camera.
    #[serde(default = "Scene::default_refractive_index")]
    pub refractive_index: Bare,

    /// Surfaces to render.
    #[serde(default)]
    pub surfaces: Vec<Surface>,
}

impl Scene {
    pub fn read_from(path: &PathBuf) -> Result<Scene> {
        let buffer = fs::read(path).with_context(|| format!("failed to read `{path:?}`"))?;
        let buffer = String::from_utf8(buffer)?;
        toml::from_str(&buffer).with_context(|| format!("failed to read a scene from `{path:?}`"))
    }

    pub const fn default_refractive_index() -> Bare {
        Bare::from(1.0)
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct Camera {
    #[serde(default = "Camera::default_location")]
    pub location: Point,

    #[serde(default)]
    pub look_at: Point,

    /// Vertical field-of-view angle, in degrees.
    #[serde(default = "Camera::default_vertical_fov", alias = "vfov")]
    pub vertical_fov: f64,

    /// Up direction.
    #[serde(default = "Camera::default_up")]
    #[schemars(with = "[f64; 3]")]
    pub up: DVec3,
}

impl Camera {
    pub const fn default_location() -> Point {
        DVec3::new(0.0, 0.0, -1.0).into()
    }

    pub const fn default_vertical_fov() -> f64 {
        45.0
    }

    pub const fn default_up() -> DVec3 {
        DVec3::new(0.0, 1.0, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            location: Self::default_location(),
            look_at: Point::default(),
            vertical_fov: Self::default_vertical_fov(),
            up: Self::default_up(),
        }
    }
}
