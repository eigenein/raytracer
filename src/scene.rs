use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

use crate::body::Body;
use crate::prelude::*;

#[derive(Deserialize)]
pub struct Scene {
    #[serde(default)]
    pub output_size: OutputSize,

    #[serde(default)]
    pub viewport: Viewport,

    #[serde(default)]
    pub bodies: Vec<Body>,
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
    pub width: f64,

    /// Distance between the projection plane and the projection point.
    pub focal_length: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1.0,
            focal_length: 1.0,
        }
    }
}
