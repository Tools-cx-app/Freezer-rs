use std::io;

use anyhow::Result;
use chrono::FixedOffset;
use flexi_logger::{DeferredNow, LogSpecification, Logger, Record};

fn log_format(
    write: &mut dyn io::Write,
    now: &mut DeferredNow,
    record: &Record<'_>,
) -> anyhow::Result<(), io::Error> {
    let utc_time = now.now();
    let time_offset = FixedOffset::east_opt(8 * 3600).unwrap();
    let time = utc_time.with_timezone(&time_offset);
    let time_format = time.format("%Y-%m-%d %H:%M:%S");
    write!(
        write,
        "[{time_format}] {}: {}",
        record.level(),
        record.args()
    )
}

pub fn log_init() -> Result<()> {
    let logger_spec = if cfg!(debug_assertions) {
        LogSpecification::debug()
    } else {
        LogSpecification::info()
    };
    Logger::with(logger_spec)
        .log_to_stdout()
        .format(log_format)
        .start()?;
    log::info!(
        "Freezer-rs v{} {}, llvm-{}, rustc-{}, build by {} at {} on {},{},{}",
        env!("CARGO_PKG_VERSION"),
        build_type(),
        env!("VERGEN_RUSTC_LLVM_VERSION"),
        env!("VERGEN_RUSTC_SEMVER"),
        env!("VERGEN_SYSINFO_USER"),
        env!("VERGEN_BUILD_TIMESTAMP"),
        env!("VERGEN_SYSINFO_NAME"),
        env!("VERGEN_SYSINFO_OS_VERSION"),
        env!("VERGEN_RUSTC_HOST_TRIPLE")
    );
    Ok(())
}

const fn build_type() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
