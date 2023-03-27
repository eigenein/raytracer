use glam::DVec3;
use image::{Rgba, RgbaImage};
use indicatif::ProgressBar;
use tracing::info;

use crate::ray::Ray;
use crate::scene::Scene;

pub fn render(scene: &Scene, into: &mut RgbaImage) {
    let progress = ProgressBar::new(into.width() as u64 * into.height() as u64);

    let eye_position = DVec3::new(0.0, 0.0, -scene.viewport.focal_length);
    info!(?eye_position);

    for x in 0..into.width() {
        let viewport_x = pixel_index_to_viewport(x, into.width(), scene.viewport.width);
        for y in 0..into.height() {
            let viewport_y = pixel_index_to_viewport(y, into.height(), scene.viewport_height());
            let ray = Ray {
                origin: eye_position,
                direction: eye_position - DVec3::new(viewport_x, viewport_y, 0.0),
            };
            let pixel = trace_ray(&ray, scene);
            into.put_pixel(x, y, pixel);
            progress.inc(1);
        }
    }

    progress.finish();
}

/// Centers the point inside the pixel and inside the entire viewport.
#[inline]
const fn pixel_index_to_viewport(x: u32, image_size: u32, viewport_size: f64) -> f64 {
    ((x as f64 + 0.5) / image_size as f64 - 0.5) * viewport_size
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
