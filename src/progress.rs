use indicatif::{ProgressBar, ProgressStyle};

use crate::prelude::*;

const STYLE: &str = "{elapsed} {wide_bar} {pos}/{len} {eta} {msg}";

pub fn new_progress(size: u64, message: &'static str) -> Result<ProgressBar> {
    let progress = ProgressBar::new(size);
    progress.set_message(message);
    progress.set_style(ProgressStyle::with_template(STYLE)?);
    Ok(progress)
}
