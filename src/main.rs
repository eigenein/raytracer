#![feature(const_fn_floating_point_arithmetic)]
#![warn(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::trivially_copy_pass_by_ref,
    clippy::map_unwrap_or,
    clippy::explicit_into_iter_loop,
    clippy::unused_self,
    clippy::needless_pass_by_value
)]

use clap::Parser;
use image::RgbaImage;
use tracing_subscriber::FmtSubscriber;

use crate::args::Args;

mod args;
mod prelude;
mod ray;
mod scene;
mod surface;
mod tracer;

use crate::prelude::*;
use crate::scene::Scene;
use crate::tracer::render;

fn main() -> Result {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;
    let args = Args::parse();
    let scene = Scene::read_from(&args.input_path)?;
    let mut output = RgbaImage::new(scene.output_size.width, scene.output_size.height);
    render(&scene, &mut output);
    output
        .save(args.output_path)
        .context("failed to save the output image")?;
    Ok(())
}
