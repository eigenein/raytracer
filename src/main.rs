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
use tracing_subscriber::FmtSubscriber;

use crate::args::{Args, Command};

mod args;
mod image;
mod prelude;

use crate::prelude::*;

fn main() -> Result {
    tracing::subscriber::set_global_default(FmtSubscriber::new())?;
    let args = Args::parse();
    match args.command {
        Command::Render(_) => {}
    }
    Ok(())
}
