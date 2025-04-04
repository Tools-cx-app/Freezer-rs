#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]
#![feature(let_chains)]
#![allow(non_snake_case)]


use anyhow::Result;

mod freezer;
mod logger;

fn main() -> Result<()> {
    logger::log_init()?;
    let mut freezer = freezer::Freezer::new()?;
    freezer.enter_looper()?;
    Ok(())
}
