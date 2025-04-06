use std::{collections::HashSet, path::PathBuf};

use v1::V1;
use v2::V2;

use super::r#enum::Mode;

mod v1;
mod v2;

pub struct Cgroup {
    pub v1: V1,
    pub v2: V2,
}

impl Cgroup {
    pub fn new() -> Self {
        Self {
            v1: V1::new(),
            v2: V2::new(),
        }
    }

    pub fn frozen(&mut self, mode: Mode, freezePath: Vec<PathBuf>, uid: HashSet<usize>) {
        log::debug!("{mode:?}");
        if let Err(e) = match mode {
            Mode::V1 => self.v1.frozen(),
            Mode::V2 => self.v2.frozen(freezePath),
            Mode::SIGSTOP => Ok(()),
        } {
            log::error!("无法写入{e}");
        }
    }
    pub fn unfrozen(&mut self, mode: Mode, freezePath: Vec<PathBuf>, uid: HashSet<usize>) {
        log::debug!("{mode:?}");
        if let Err(e) = match mode {
            Mode::V1 => self.v1.frozen(),
            Mode::V2 => self.v2.unfrozen(freezePath),
            Mode::SIGSTOP => Ok(()),
        } {
            log::error!("无法写入{e}");
        }
    }
}
