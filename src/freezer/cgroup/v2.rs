use std::{collections::HashSet, fs::write, path::PathBuf, str::FromStr};

use anyhow::Result;

use crate::freezer::r#enum::V2Mode;

pub struct V2 {
    uid: usize,
    mode: Option<V2Mode>,
}

impl V2 {
    pub fn new() -> Self {
        Self { uid: 0, mode: None }
    }

    pub fn set_mode(&mut self, mode: V2Mode) {
        match mode {
            V2Mode::Uid => self.mode = Some(V2Mode::Uid),
            V2Mode::Frozen => self.mode = Some(V2Mode::Frozen),
        };
    }

    pub fn frozen(&self, path: Vec<PathBuf>, uid: HashSet<usize>) -> Result<()> {
        //let path = PathBuf::from_str("/sys/fs/cgroup/uid_0/cgroup.freeze")?;
        for p in path {
            if !p.exists() {
                log::error!("{}不存在", p.display());
                return Ok(());
            }
            write(&p, "1".as_bytes())?;
            log::info!("{}已冻结", p.display());
        }
        for u in uid {
            write("/sys/fs/cgroup/frozen/cgroup.freeze", [u as u8])?;
        }
        Ok(())
    }

    pub fn unfrozen(&self) -> Result<()> {
        let path = PathBuf::from_str("/dev/freezer/unfrozen/cgroup.procs")?;
        if !path.exists() {
            log::error!("{}不存在", path.display());
            return Ok(());
        }
        write(path, [0 as u8])?;
        Ok(())
    }
}
