use glam::DVec3;
use image::{Rgba, RgbaImage};
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
                direction: eye_position - viewport_point,
            };
            let pixel = trace_ray(&ray, scene);
            into.put_pixel(x, y, pixel);
            progress.inc(1);
        }
    }

    progress.finish();
}

#[inline]
fn trace_ray(ray: &Ray, in_: &Scene) -> Rgba<u8> {
    for surface in &in_.surfaces {
        if let Some(hit) = surface.hit(ray) {
            return Rgba::from([
                (hit.normal.x * 255.0) as u8,
                (hit.normal.y * 255.0) as u8,
                (hit.normal.z * 255.0) as u8,
                255,
            ]);
        }
    }
    Rgba::from([0, 0, 0, 255])
}
