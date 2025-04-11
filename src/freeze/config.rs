use std::{collections::HashSet, fs};

use anyhow::{Context, Result};
use serde::Deserialize;

use super::FreezeMode;

#[derive(Deserialize)]
pub struct Config {
    pub mode: FreezeMode,
    pub whitelist: HashSet<String>,
}

impl Config {
    pub fn load_config(&mut self) -> Result<Self> {
        let file = fs::read_to_string("/storage/emulated/0/Android/freezer.toml").context("无法读取配置文件")?;
        let content: Self = toml::from_str(file.as_str())?;

        Ok(content)
    }
}
