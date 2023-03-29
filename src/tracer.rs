use std::cmp::Ordering;

use glam::{DVec3, DVec4};
use image::{Rgba, RgbaImage};
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;

use crate::prelude::*;
use crate::ray::Ray;
use crate::scene::Scene;

pub fn render(scene: &Scene, into: &mut RgbaImage) -> Result {
    // Vectors to convert the image's pixel coordinates to the viewport's ones:
    let x_vector = DVec3::new(scene.viewport.width, 0.0, 0.0) / into.width() as f64;
    let y_vector = DVec3::new(0.0, -x_vector.x, 0.0);
    info!(?x_vector, ?y_vector);

    let viewport_center = DVec3::new(0.0, 0.0, -scene.viewport.distance);
    info!(?viewport_center);

    let eye_position = DVec3::new(0.0, 0.0, -scene.viewport.focal_length - scene.viewport.distance);
    info!(?eye_position);

    let half_image_width = into.width() as f64 / 2.0;
    let half_image_height = into.height() as f64 / 2.0;
    let samples_per_pixel = scene.samples_per_pixel as f64;
    info!(scene.samples_per_pixel);

    let progress = ProgressBar::new(into.height() as u64);
    progress.set_style(ProgressStyle::with_template(
        "{elapsed} {wide_bar:.cyan/blue} {pos}/{len} {eta} {msg}",
    )?);

    for y in 0..into.height() {
        for x in 0..into.width() {
            // Sum multiple samples for antialiasing:
            let color = (0..scene.samples_per_pixel)
                .map(|_| {
                    let mut image_x = x as f64 - half_image_width;
                    let mut image_y = y as f64 - half_image_height;
                    if scene.samples_per_pixel != 1 {
                        image_x += fastrand::f64() - 0.5;
                        image_y += fastrand::f64() - 0.5;
                    }
                    let viewport_point = viewport_center + image_x * x_vector + image_y * y_vector;
                    trace_ray(
                        &Ray {
                            origin: eye_position,
                            direction: viewport_point - eye_position,
                        },
                        scene,
                    )
                })
                .sum::<DVec4>()
                / samples_per_pixel;
            into.put_pixel(
                x,
                y,
                Rgba::from([
                    (color.x * 255.0).round() as u8,
                    (color.y * 255.0).round() as u8,
                    (color.z * 255.0).round() as u8,
                    (color.w * 255.0).round() as u8,
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
fn trace_ray(ray: &Ray, in_: &Scene) -> DVec4 {
    in_.surfaces
        .iter()
        .filter_map(|surface| surface.hit(ray, 0.0..f64::INFINITY))
        .min_by(|hit_1, hit_2| {
            hit_1
                .time
                .partial_cmp(&hit_2.time)
                .unwrap_or(Ordering::Equal)
        })
        .map_or(DVec4::new(0.0, 0.0, 0.0, 1.0), |hit| {
            DVec4::new(hit.normal.x.abs(), hit.normal.y.abs(), hit.normal.z.abs(), 1.0)
        })
}
