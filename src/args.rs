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

        /// Gamma for the post-correction.
        ///
        /// It is applied after the conversion to 3-float RGB
        /// but before the conversion to the 16-bit RGB.
        #[arg(short = 'g', long = "gamma", default_value = "1.0")]
        gamma: f64,

        /// Number of rendering threads (`0` for automatic choice).
        #[arg(short = 't', long = "threads", default_value = "0")]
        n_threads: usize,

        /// Maximal number of surfaces in a single leaf of the bounding volume hierarchy.
        #[arg(long, default_value = "4")]
        max_bvh_leaf_size: usize,

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
    /// Number of random rays per pixel that get averaged to obtain a final color.
    #[arg(short = 's', long = "samples", default_value = "1", value_parser = value_parser!(u32).range(1..))]
    pub samples_per_pixel: u32,

    /// Maximum number of ray bounces of the scene's surfaces.
    ///
    /// Each ray's bounce count gets decreased by one when the ray gets scattered.
    /// Once it reaches zero, no scattered rays get traced any more.
    #[arg(short = 'b', long = "max-bounces", default_value = "5", value_parser = value_parser!(u16).range(1..))]
    pub n_max_bounces: u16,

    /// Minimal distance from a ray's origin point to a possible hit.
    ///
    /// This is needed to prevent collision of the ray with its own origin surface.
    #[arg(long, default_value = "0.000001")]
    pub min_hit_distance: f64,

    /// Minimal total attenuation to continue tracing a ray.
    ///
    /// When the total attenuation drops below the setting, no scattered rays get traced any more.
    /// This saves some time because low attenuation doesn't contribute enough to the final intensity.
    ///
    /// This helps a lot in, for example, a foggy environment.
    #[arg(long, default_value = "0.000001")]
    pub min_attenuation: f64,
}
