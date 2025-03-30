#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

use anyhow::Result;
use inotify::{Inotify, WatchMask};

mod freezer;
mod logger;

fn main() -> Result<()> {
    logger::log_init()?;
    let mut freezer = freezer::Freezer::new()?;
    let mut inotify = Inotify::init()?;
    inotify.watches().add("/dev/input", WatchMask::ACCESS)?;
    loop {
        inotify.read_events_blocking(&mut [0; 1024])?;
        freezer.get_visible_app();
    }
    Ok(())
}
