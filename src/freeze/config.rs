use std::{collections::HashSet, fs};

use anyhow::{Context, Result};
use serde::Deserialize;

use super::FreezeMode;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mode: FreezeMode,
    pub whitelist: HashSet<String>,
}

impl Config {
    pub fn load_config(&mut self) -> Result<()> {
        let file = fs::read_to_string("/data/freezer.toml").context("无法读取配置文件")?;
        let content: Self = toml::from_str(file.as_str()).context("配置文件错误")?;

        self.mode = content.mode;
        self.whitelist = content.whitelist;
        Ok(())
    }
}
