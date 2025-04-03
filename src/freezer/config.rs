use std::{collections::HashSet, fs};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub white_list: HashSet<String>,
}

impl ConfigData {
    pub fn new() -> Result<Self> {
        let context = fs::read_to_string("/storage/emulated/0/Android/freezer.toml")
            .with_context(|| "无法读取配置文件")?;
        let toml: Self = toml::from_str(context.as_str()).with_context(|| "无法转换配置文件")?;
        Ok(toml)
    }
}
