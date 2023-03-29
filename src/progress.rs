use indicatif::{ProgressBar, ProgressStyle};

use crate::prelude::*;

pub fn new_progress(size: u64, message: &'static str) -> Result<ProgressBar> {
    let progress = ProgressBar::new(size);
    progress.set_message(message);
    progress.set_style(ProgressStyle::with_template(
        "{elapsed} {wide_bar:.cyan/blue} {pos}/{len} {eta} {msg}",
    )?);
    Ok(progress)
}
