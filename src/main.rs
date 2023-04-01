#![feature(const_fn_floating_point_arithmetic)]
#![feature(portable_simd)]
#![warn(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::trivially_copy_pass_by_ref,
    clippy::map_unwrap_or,
    clippy::explicit_into_iter_loop,
    clippy::unused_self,
    clippy::needless_pass_by_value
)]

use std::f64::consts::E;

use ::image::Rgb;
use clap::Parser;
use glam::DVec3;
use itertools::{iproduct, izip};
use tracing_subscriber::FmtSubscriber;

use crate::args::Args;
use crate::image::Rgb16Image;

mod aabb;
mod args;
mod constants;
mod hit;
mod image;
mod material;
mod math;
mod prelude;
mod progress;
mod ray;
mod scene;
mod surface;
mod tracer;
mod viewport;

use crate::prelude::*;
use crate::progress::new_progress;
use crate::scene::Scene;
use crate::tracer::Tracer;

fn main() -> Result {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;
    let args = Args::parse();
    let scene = Scene::read_from(&args.input_path)?;
    let pixels =
        Tracer::new(scene, args.tracer_options).trace(args.output_width, args.output_height)?;
    convert_pixels_to_image(args.output_width, args.output_height, pixels, args.logarithmic_light)?
        .save(args.output_path)
        .context("failed to save the output image")?;
    Ok(())
}

fn convert_pixels_to_image(
    output_width: u32,
    output_height: u32,
    pixels: Vec<DVec3>,
    logarithmic_light: bool,
) -> Result<Rgb16Image> {
    let max_intensity = pixels
        .iter()
        .map(|pixel| pixel.max_element())
        .max_by(|lhs, rhs| lhs.total_cmp(rhs))
        .unwrap_or(1.0)
        .max(1.0);
    info!(max_intensity);

    let scale = if logarithmic_light {
        (E - 1.0) / max_intensity
    } else {
        1.0 / max_intensity
    };
    info!(scale);

    let mut image = Rgb16Image::new(output_width, output_height);
    let progress = new_progress(pixels.len() as u64, "converting to image")?;
    for ((y, x), pixel) in izip!(iproduct!(0..output_height, 0..output_width), pixels) {
        // Scale to the maximum intensity:
        let color = if logarithmic_light {
            let color = pixel * scale + 1.0;
            DVec3::new(color.x.ln(), color.y.ln(), color.z.ln())
        } else {
            pixel * scale
        };
        assert!(color.cmpge(DVec3::ZERO).all());
        assert!(color.cmple(DVec3::ONE).all());
        // Scale to the image sub-pixels:
        let color = color * u16::MAX as f64;
        // And finally, prepare for casting:
        let color = color.round();

        image.put_pixel(x, y, Rgb::from([color.x as u16, color.y as u16, color.z as u16]));
        progress.inc(1);
    }
    progress.finish();
    Ok(image)
}
