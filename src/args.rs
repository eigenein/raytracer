use std::path::PathBuf;

use clap::{value_parser, Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    /// Trace and render the scene.
    Render {
        /// Scene configuration, in TOML format.
        #[arg(value_name = "INPUT")]
        input_path: PathBuf,

        /// Output image path.
        #[arg(value_name = "OUTPUT")]
        output_path: PathBuf,

        /// Output image width.
        #[arg(long = "width", default_value = "1920", value_parser = value_parser!(u32).range(1..))]
        output_width: u32,

        /// Output image height.
        #[arg(long = "height", default_value = "1080", value_parser = value_parser!(u32).range(1..))]
        output_height: u32,

        #[arg(short = 'g', long = "gamma", default_value = "1.0")]
        gamma: f64,

        #[clap(flatten)]
        tracer_options: TracerOptions,
    },

    /// Print the scene JSON schema.
    Schema,
}

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser)]
pub struct TracerOptions {
    /// Samples per pixel that get averaged for the antialiasing.
    #[arg(short = 's', long = "samples", default_value = "1", value_parser = value_parser!(u16).range(1..))]
    pub samples_per_pixel: u16,

    /// Maximum number of ray bounces of the scene's surfaces.
    #[arg(short = 'b', long = "max-bounces", default_value = "5", value_parser = value_parser!(u16).range(1..))]
    pub n_max_bounces: u16,

    /// Minimal distance from a ray's origin point to a possible hit.
    /// This is needed to prevent collision of the ray with its own origin surface.
    #[arg(long, default_value = "0.000001")]
    pub min_hit_distance: f64,
}
