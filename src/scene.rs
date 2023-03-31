use std::fs;
use std::path::PathBuf;

use glam::DVec3;
use serde::Deserialize;

use crate::prelude::*;
use crate::surface::Surface;

/// A scene to render.
///
/// This is a root object in a scene TOML file.
#[derive(Deserialize)]
pub struct Scene {
    #[serde(default)]
    pub camera: Camera,

    /// Scene background and ambient color.
    #[serde(default = "Scene::default_ambient_color")]
    pub ambient_color: DVec3,

    /// Scene medium refractive index.
    ///
    /// This index is assigned for the primary incident rays originating
    /// from the camera.
    #[serde(default = "Scene::default_refractive_index")]
    pub refractive_index: f64,

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

    pub const fn default_ambient_color() -> DVec3 {
        DVec3::ZERO
    }

    pub const fn default_refractive_index() -> f64 {
        1.0
    }
}

#[derive(Deserialize)]
pub struct Camera {
    #[serde(default = "Camera::default_location")]
    pub location: DVec3,

    #[serde(default)]
    pub look_at: DVec3,

    /// Vertical field-of-view angle, in degrees.
    #[serde(default = "Camera::default_vertical_fov", alias = "vfov")]
    pub vertical_fov: f64,

    /// Up direction.
    #[serde(default = "Camera::default_up")]
    pub up: DVec3,
}

impl Camera {
    pub const fn default_location() -> DVec3 {
        DVec3::new(0.0, 0.0, -1.0)
    }

    pub const fn default_vertical_fov() -> f64 {
        90.0
    }

    pub const fn default_up() -> DVec3 {
        DVec3::new(0.0, 1.0, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            location: Self::default_location(),
            look_at: DVec3::default(),
            vertical_fov: Self::default_vertical_fov(),
            up: Self::default_up(),
        }
    }
}
