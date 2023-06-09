use std::fs;
use std::path::PathBuf;

use schemars::JsonSchema;
use serde::Deserialize;

use crate::math::vec3::Vec3;
use crate::physics::optics::material::emittance::Emittance;
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
    #[serde(default, alias = "ambient_spectrum")]
    pub ambient_emittance: Emittance,

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
}

#[derive(Deserialize, JsonSchema)]
pub struct Camera {
    /// Camera location point.
    #[serde(default = "Camera::default_location")]
    pub location: Vec3,

    /// Point to look at.
    #[serde(default)]
    pub look_at: Vec3,

    /// Vertical field-of-view angle, in degrees.
    #[serde(default = "Camera::default_vertical_fov", alias = "vfov")]
    pub vertical_fov: f64,

    /// Up **direction** (not a point).
    #[serde(default = "Camera::default_up")]
    pub up: Vec3,
}

impl Camera {
    pub const fn default_location() -> Vec3 {
        Vec3::new(0.0, 0.0, -1.0)
    }

    pub const fn default_vertical_fov() -> f64 {
        45.0
    }

    pub const fn default_up() -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            location: Self::default_location(),
            look_at: Vec3::default(),
            vertical_fov: Self::default_vertical_fov(),
            up: Self::default_up(),
        }
    }
}
