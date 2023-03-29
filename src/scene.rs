use std::fs;
use std::path::PathBuf;

use glam::DVec4;
use serde::Deserialize;

use crate::prelude::*;
use crate::surface::Surface;

#[derive(Deserialize)]
pub struct Scene {
    /// Output image size.
    #[serde(default)]
    pub output_size: OutputSize,

    /// Projection viewport.
    #[serde(default)]
    pub viewport: Viewport,

    /// Scene background and ambient color.
    #[serde(default = "Scene::default_ambient_color")]
    pub ambient_color: DVec4,

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

    pub const fn default_ambient_color() -> DVec4 {
        DVec4::W
    }
}

#[derive(Deserialize)]
pub struct OutputSize {
    /// Output image width, in pixels.
    #[serde(default = "OutputSize::default_width")]
    pub width: u32,

    /// Output image height, in pixels.
    #[serde(default = "OutputSize::default_height")]
    pub height: u32,
}

impl OutputSize {
    pub const fn default_width() -> u32 {
        1920
    }

    pub const fn default_height() -> u32 {
        1080
    }
}

impl Default for OutputSize {
    fn default() -> Self {
        Self {
            width: Self::default_width(),
            height: Self::default_height(),
        }
    }
}

#[derive(Deserialize)]
pub struct Viewport {
    /// Viewport width, in meters.
    #[serde(default = "Viewport::default_width")]
    pub width: f64,

    /// Distance between the projection plane and the world center, in meters.
    #[serde(default = "Viewport::default_distance")]
    pub distance: f64,

    /// Distance between the projection plane and the projection point, in meters.
    #[serde(default = "Viewport::default_focal_length")]
    pub focal_length: f64,
}

impl Viewport {
    pub const fn default_width() -> f64 {
        1.0
    }

    pub const fn default_distance() -> f64 {
        1.0
    }

    pub const fn default_focal_length() -> f64 {
        1.0
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: Self::default_width(),
            distance: Self::default_distance(),
            focal_length: Self::default_focal_length(),
        }
    }
}
