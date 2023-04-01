use std::path::PathBuf;

use clap::{value_parser, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    /// Scene configuration, in TOML format.
    #[arg(value_name = "INPUT")]
    pub input_path: PathBuf,

    /// Output image path.
    #[arg(value_name = "OUTPUT")]
    pub output_path: PathBuf,

    /// Output image width.
    #[arg(long = "width", default_value = "1920", value_parser = value_parser!(u32).range(1..))]
    pub output_width: u32,

    /// Output image height.
    #[arg(long = "height", default_value = "1080", value_parser = value_parser!(u32).range(1..))]
    pub output_height: u32,

    /// Use logarithmic scaling when converting pixel's light intensity to a color value.
    #[arg(short = 'l', long = "log-light")]
    pub logarithmic_light: bool,

    #[clap(flatten)]
    pub tracer_options: TracerOptions,
}

#[derive(Parser, Debug)]
pub struct TracerOptions {
    /// Samples per pixel that get averaged for the antialiasing.
    #[arg(short = 's', long = "samples", default_value = "1", value_parser = value_parser!(u16).range(1..))]
    pub samples_per_pixel: u16,

    /// Maximum number of rays traced in depth.
    #[arg(short = 'd', long = "depth", default_value = "5", value_parser = value_parser!(u16).range(1..))]
    pub max_depth: u16,
}
