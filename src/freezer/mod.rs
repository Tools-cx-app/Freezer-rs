use std::{collections::HashSet, process::Command};

use anyhow::Result;
use app::App;
use lazy_static::lazy_static;
use regex::Regex;

mod app;

lazy_static! {
    static ref COMPONENT_RE: Regex = Regex::new(r".*\{([^/]+)/").unwrap();
}
pub struct Freezer {
    app: App,
}

impl Freezer {
    pub fn new() -> Result<Self> {
        Ok(Self { app: App::new()? })
    }

    pub fn get_visible_app(&mut self) -> HashSet<usize> {
        let output = Command::new("/system/bin/cmd")
            .args(&["activity", "stack", "list"])
            .output()
            .expect("Failed to execute command");
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut lines = output_str.lines();
        let mut cur_foreground_app = HashSet::new();
        while let Some(line) = lines.next() {
            // 处理桌面应用
            /*if !self.app.has_home_package() && line.contains("mActivityType=home") {
                if let Some(next_line) = lines.next() {
                    if let Some(caps) = COMPONENT_RE.captures(next_line) {
                        let package = caps.get(1).unwrap().as_str();
                        managed_app.update_home_package(package);
                    }
                }
            }*/

            if line.starts_with("  taskId=") && line.contains("visible=true") {
                if let Some(caps) = COMPONENT_RE.captures(line) {
                    let package = caps.get(1).unwrap().as_str();
                    if self.app.contains(package) {
                        let uid = self.app.get_uid(package);
                        log::debug!("{:?}", uid);
                        if !self.app.is_whitelist(uid) {
                            cur_foreground_app.insert(uid);
                        }
                    }
                }
            }
        }
        cur_foreground_app
    }
}
