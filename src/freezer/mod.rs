use std::{collections::HashSet, process::Command};

use anyhow::Result;
use app::App;
use inotify::{Inotify, WatchMask};
use lazy_static::lazy_static;
use regex::Regex;

mod app;
mod config;

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

    fn get_visible_app(&mut self) -> HashSet<usize> {
        let output = Command::new("/system/bin/cmd")
            .args(["activity", "stack", "list"])
            .output()
            .expect("无法执行cmd activity stack list");
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines = output_str.lines();
        let mut cur_foreground_app = HashSet::new();
        for line in lines {
            if line.starts_with("  taskId=") && line.contains("visible=true") {
                if let Some(caps) = COMPONENT_RE.captures(line) {
                    let package = caps.get(1).unwrap().as_str();
                    if self.app.contains(package) {
                        let uid = self.app.get_uid(package);
                        let pids = self.app.get_pids(package).unwrap();
                        if !self.app.is_whitelist(uid) {
                            cur_foreground_app.insert(uid);
                        }
                    }
                }
            }
        }
        cur_foreground_app
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let mut inotify = Inotify::init()?;
        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;
        let config = config::ConfigData::new()?;
        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            let visible_app = self.get_visible_app();

            #[cfg(debug_assertions)]
            {
                log::debug!("当前顶层应用uid: {:?}", visible_app);
            }
        }
    }
}
