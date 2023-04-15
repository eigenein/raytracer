#![allow(incomplete_features)]
#![feature(const_cmp)]
#![feature(const_convert)]
#![feature(const_float_classify)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_precise_live_drops)]
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(let_chains)]
#![feature(repr_simd)]
#![feature(test)]
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

use clap::Parser;
use schemars::schema_for;
use tracing_subscriber::FmtSubscriber;

use crate::args::{Args, Command};
use crate::color::rgb::RgbColor;
use crate::color::xyz::XyzColor;
use crate::image::Rgb16Image;

mod args;
mod color;
mod image;
mod math;
mod physics;
mod prelude;
mod scene;
mod surface;
mod tracer;

use crate::prelude::*;
use crate::scene::Scene;
use crate::tracer::bvh::Bvh;
use crate::tracer::progress::new_progress;
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
            n_threads,
            max_bvh_leaf_size,
        } => {
            rayon::ThreadPoolBuilder::new()
                .num_threads(n_threads)
                .build_global()?;
            info!(current_num_threads = rayon::current_num_threads());

            let mut scene = Scene::read_from(&input_path)?;
            info!(n_surfaces = scene.surfaces.len(), "building bounded volume hierarchy…");
            let bvh = Bvh::new(&mut scene.surfaces, max_bvh_leaf_size);

            let pixels = Tracer::new(
                bvh,
                scene.ambient_emittance,
                scene.camera,
                tracer_options,
                output_width,
                output_height,
            )
            .trace()?;
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
    rows: Vec<(u32, Vec<XyzColor>)>,
    gamma: f64,
) -> Result<Rgb16Image> {
    let max_intensity = rows
        .iter()
        .flat_map(|(_, row)| row)
        .map(|pixel| pixel.max_element())
        .max_by(|lhs, rhs| lhs.total_cmp(rhs))
        .unwrap_or(1.0)
        .max(1.0);
    let scale = 1.0 / max_intensity;
    info!(max_intensity, scale);

    let mut image = Rgb16Image::new(output_width, output_height);
    let progress = new_progress(rows.len() as u64, "converting to image")?;
    for (y, row) in rows {
        for (x, color) in row.into_iter().enumerate() {
            let srgb_color = RgbColor::from(color * scale);
            image.put_pixel(x as u32, y, srgb_color.apply_gamma(gamma).into());
        }
        progress.inc(1);
    }
    progress.finish();
    Ok(image)
}
