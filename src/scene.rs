use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::prelude::*;
use crate::surface::Surface;

#[derive(Deserialize)]
pub struct Scene {
    /// Output image size.
    #[serde(default)]
    pub output_size: OutputSize,

    /// Projection viewport.
    /// The eye is located at `(0.0, 0.0, -focal_length)`.
    pub viewport: Viewport,

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

    pub const fn viewport_height(&self) -> f64 {
        self.viewport.width / self.output_size.width as f64 * self.output_size.height as f64
    }
}

#[derive(Deserialize)]
pub struct OutputSize {
    /// Output image width, in pixels.
    pub width: u32,

    /// Output image height, in pixels.
    pub height: u32,
}

impl Default for OutputSize {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}

#[derive(Deserialize)]
pub struct Viewport {
    /// Viewport width, in meters.
    #[serde(default = "Viewport::default_width")]
    pub width: f64,

    /// Distance between the projection plane and the projection point, in meters.
    #[serde(default = "Viewport::default_focal_length")]
    pub focal_length: f64,
}

impl Viewport {
    pub const fn default_width() -> f64 {
        1.0
    }

    pub const fn default_focal_length() -> f64 {
        1.0
    }
}
