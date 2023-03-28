use std::cmp::Ordering;

use colorsys::ColorAlpha;
use glam::DVec3;
use image::RgbaImage;
use indicatif::ProgressBar;
use tracing::info;

use crate::ray::Ray;
use crate::scene::Scene;

pub fn render(scene: &Scene, into: &mut RgbaImage) {
    let x_vector = DVec3::new(scene.viewport.width, 0.0, 0.0) / into.width() as f64;
    let y_vector = DVec3::new(0.0, -x_vector.x, 0.0);
    info!(?x_vector, ?y_vector);

    let viewport_center = DVec3::new(0.0, 0.0, -scene.viewport.distance);
    info!(?viewport_center);

    let eye_position = DVec3::new(0.0, 0.0, -scene.viewport.focal_length - scene.viewport.distance);
    info!(?eye_position);

    let progress = ProgressBar::new(into.width() as u64 * into.height() as u64);
    for x in 0..into.width() {
        for y in 0..into.height() {
            let viewport_point = viewport_center
                + (x as f64 - into.width() as f64 / 2.0) * x_vector
                + (y as f64 - into.height() as f64 / 2.0) * y_vector;
            let ray = Ray {
                origin: eye_position,
                direction: viewport_point - eye_position,
            };
            let pixel = trace_ray(&ray, scene);
            let pixel = image::Rgba::from([
                pixel.red().round() as u8,
                pixel.green().round() as u8,
                pixel.blue().round() as u8,
                pixel.alpha().round() as u8,
            ]);
            into.put_pixel(x, y, pixel);
            progress.inc(1);
        }
    }

    progress.finish();
}

#[inline]
fn trace_ray(ray: &Ray, in_: &Scene) -> colorsys::Rgb {
    in_.surfaces
        .iter()
        .filter_map(|surface| surface.hit(ray, 0.0..f64::INFINITY))
        .min_by(|hit_1, hit_2| {
            hit_1
                .time
                .partial_cmp(&hit_2.time)
                .unwrap_or(Ordering::Equal)
        })
        .map(|hit| {
            colorsys::Rgb::from([
                hit.normal.x.abs() * 255.0,
                hit.normal.y.abs() * 255.0,
                hit.normal.z.abs() * 255.0,
                255.0,
            ])
        })
        .unwrap_or(colorsys::Rgb::from([0.0, 0.0, 0.0, 255.0]))
}
