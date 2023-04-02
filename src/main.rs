#![feature(const_convert)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_trait_impl)]
#![feature(let_chains)]
#![feature(portable_simd)]
#![warn(
    clippy::all,
    clippy::explicit_into_iter_loop,
    clippy::manual_let_else,
    clippy::map_unwrap_or,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value,
    clippy::trivially_copy_pass_by_ref,
    clippy::unused_self
)]

use ::image::Rgb;
use clap::Parser;
use glam::DVec3;
use schemars::schema_for;
use tracing_subscriber::FmtSubscriber;

use crate::args::{Args, Command};
use crate::image::Rgb16Image;

mod aabb;
mod args;
mod constants;
mod hit;
mod image;
mod lighting;
mod material;
mod math;
mod prelude;
mod progress;
mod ray;
mod refraction;
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
    match args.command {
        Command::Render {
            input_path,
            tracer_options,
            output_width,
            output_height,
            gamma,
            output_path,
        } => {
            let scene = Scene::read_from(&input_path)?;
            let pixels = Tracer::new(scene, tracer_options).trace(output_width, output_height)?;
            convert_pixels_to_image(output_width, output_height, pixels, gamma)?
                .save(output_path)
                .context("failed to save the output image")?;
        }

        Command::Schema => {
            println!("{}", serde_json::to_string_pretty(&schema_for!(Scene))?);
        }
    }
    Ok(())
}

fn convert_pixels_to_image(
    output_width: u32,
    output_height: u32,
    pixels: Vec<(u32, u32, DVec3)>,
    gamma: f64,
) -> Result<Rgb16Image> {
    let max_intensity = pixels
        .iter()
        .map(|(_, _, pixel)| pixel.max_element())
        .max_by(|lhs, rhs| lhs.total_cmp(rhs))
        .unwrap_or(1.0)
        .max(1.0);
    info!(max_intensity);

    let scale = 1.0 / max_intensity;
    info!(scale);

    let mut image = Rgb16Image::new(output_width, output_height);
    let progress = new_progress(pixels.len() as u64, "converting to image")?;
    for (x, y, pixel) in pixels {
        // Scale to the maximum luminance:
        let color = (pixel * scale).clamp(DVec3::ZERO, DVec3::ONE);
        // Apply the gamma correction:
        let color = color.powf(gamma);
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
