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

    #[clap(flatten)]
    pub tracer_options: TracerOptions,
}

#[derive(Parser, Debug)]
pub struct TracerOptions {
    /// Samples per pixel that get averaged for the antialiasing.
    /// When equals to `1`, no randomization for ray direction is applied.
    #[arg(short, long, default_value = "1", value_parser = value_parser!(u16).range(1..))]
    pub samples_per_pixel: u16,
}
