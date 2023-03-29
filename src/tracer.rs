use std::cmp::Ordering;

use glam::{DVec3, DVec4};
use image::{Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;

use crate::args::TracerOptions;
use crate::prelude::*;
use crate::ray::Ray;
use crate::scene::Scene;

pub struct Tracer {
    pub scene: Scene,
    pub options: TracerOptions,
}

impl Tracer {
    pub const fn new(scene: Scene, options: TracerOptions) -> Self {
        Self { scene, options }
    }

    pub fn render(&self, into: &mut RgbImage) -> Result {
        // Vectors to convert the image's pixel coordinates to the viewport's ones:
        let x_vector = DVec3::new(self.scene.viewport.width, 0.0, 0.0) / into.width() as f64;
        let y_vector = DVec3::new(0.0, -x_vector.x, 0.0);
        info!(?x_vector, ?y_vector);

        let viewport_center = DVec3::new(0.0, 0.0, -self.scene.viewport.distance);
        info!(?viewport_center);

        let eye_position =
            DVec3::new(0.0, 0.0, -self.scene.viewport.focal_length - self.scene.viewport.distance);
        info!(?eye_position);

        let half_image_width = into.width() as f64 / 2.0;
        let half_image_height = into.height() as f64 / 2.0;
        let samples_per_pixel = self.options.samples_per_pixel as f64;
        info!(self.options.samples_per_pixel);

        let progress = ProgressBar::new(into.height() as u64);
        progress.set_style(ProgressStyle::with_template(
            "{elapsed} {wide_bar:.cyan/blue} {pos}/{len} {eta} {msg}",
        )?);

        for y in 0..into.height() {
            for x in 0..into.width() {
                // Sum multiple samples for antialiasing:
                let color = (0..self.options.samples_per_pixel)
                    .map(|_| {
                        let mut image_x = x as f64 - half_image_width;
                        let mut image_y = y as f64 - half_image_height;
                        if self.options.samples_per_pixel != 1 {
                            image_x += fastrand::f64() - 0.5;
                            image_y += fastrand::f64() - 0.5;
                        }
                        let viewport_point =
                            viewport_center + image_x * x_vector + image_y * y_vector;
                        self.trace_ray(&Ray::by_two_points(eye_position, viewport_point))
                    })
                    .sum::<DVec4>()
                    / samples_per_pixel;
                into.put_pixel(
                    x,
                    y,
                    Rgb::from([
                        (color.x * 255.0).round() as u8,
                        (color.y * 255.0).round() as u8,
                        (color.z * 255.0).round() as u8,
                    ]),
                );
            }
            progress.inc(1);
        }
        progress.finish();

        Ok(())
    }

    /// Trace the ray and return the resulting color.
    #[inline]
    fn trace_ray(&self, ray: &Ray) -> DVec4 {
        self.scene
            .surfaces
            .iter()
            .filter_map(|surface| surface.hit(ray, 0.0..f64::INFINITY))
            .min_by(|hit_1, hit_2| {
                hit_1
                    .time
                    .partial_cmp(&hit_2.time)
                    .unwrap_or(Ordering::Equal)
            })
            .map_or(self.scene.ambient_color, |hit| {
                DVec4::new(hit.normal.x.abs(), hit.normal.y.abs(), hit.normal.z.abs(), 1.0)
            })
    }
}
