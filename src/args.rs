use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Args {
    /// Scene configuration, in TOML format.
    #[arg(value_name = "INPUT")]
    pub input_path: PathBuf,

    /// Output image path.
    #[arg(value_name = "OUTPUT")]
    pub output_path: PathBuf,
}
