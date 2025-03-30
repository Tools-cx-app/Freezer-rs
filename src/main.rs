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
