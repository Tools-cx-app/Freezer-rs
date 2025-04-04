use std::{fs::write, path::PathBuf, str::FromStr};

use anyhow::Result;

pub struct V1 {
    uid: usize,
}

impl V1 {
    pub fn new() -> Self {
        Self { uid: 0 }
    }

    pub fn frozen(&self) -> Result<()> {
        let path = PathBuf::from_str("/dev/freezer/frozen/cgroup.procs")?;
        if !path.exists() {
            log::error!("{}不存在", path.display());
            return Ok(());
        }
        write(path, [self.uid as u8])?;
        Ok(())
    }

    pub fn unfrozen(&self) -> Result<()> {
        let path = PathBuf::from_str("/dev/freezer/unfrozen/cgroup.procs")?;
        if !path.exists() {
            log::error!("{}不存在", path.display());
            return Ok(());
        }
        write(path, [self.uid as u8])?;
        Ok(())
    }
}
