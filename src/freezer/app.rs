use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PM_REGEX: Regex = Regex::new(r"package:(?<pkg>\S+) uid:(?<uid>\d+)").unwrap();
}

pub struct App {
    pids: HashMap<String, usize>,
    packages: HashMap<String, usize>,
    whitelist: HashSet<usize>,
}

impl App {
    pub fn new() -> Result<Self> {
        let path = "/data/system/packages.list";
        let file = std::fs::File::open(path).with_context(|| format!("未能打开 {path}"))?;
        let mut apps = HashMap::new();
        for line in std::io::BufRead::lines(std::io::BufReader::new(file)) {
            let line = line.with_context(|| "读取行时出错")?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 2 {
                continue;
            }

            let uid = parts[1]
                .parse::<usize>()
                .with_context(|| format!("无效的UID格式: {}", parts[1]))?;

            if uid < 10000 {
                continue;
            }
            apps.insert(parts[0].to_string(), uid);
        }
        Ok(Self {
            pids: HashMap::new(),
            packages: apps,
            whitelist: HashSet::new(),
        })
    }

    pub fn get_pids(&mut self, package: &str) -> Result<HashMap<String, usize>> {
        let proc_dir = fs::read_dir("/proc").with_context(|| "无法读取/proc/")?;
        for entry in proc_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let file_name = entry.file_name();
            let pid_str = match file_name.to_str() {
                Some(s) => s,
                None => continue,
            };

            let pid = match pid_str.parse::<usize>() {
                Ok(p) => p,
                Err(_) => continue,
            };

            if pid <= 100 {
                continue;
            }

            let cmdline_path = Path::new("/proc").join(pid_str).join("cmdline");
            let cmdline = match fs::read(&cmdline_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let cmdline_str = String::from_utf8_lossy(&cmdline);
            if cmdline_str.starts_with(package) {
                self.pids.insert(package.to_string(), pid);
            }
        }
        Ok(self.pids.clone())
    }

    pub fn add_whitelist(&mut self, packages: HashSet<String>) {
        for package in packages {
            self.whitelist.insert(self.get_uid(package.as_str()));
        }
        #[cfg(debug_assertions)]
        {
            log::debug!("白名单应用:{:?}", self.whitelist);
        }
    }

    pub fn is_backstage(&self, package: &str) -> Result<bool> {
        let proc_dir = fs::read_dir("/proc").with_context(|| "无法读取/proc/")?;
        for entry in proc_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let file_name = entry.file_name();
            let pid_str = match file_name.to_str() {
                Some(s) => s,
                None => continue,
            };

            return Ok(pid_str.contains(package));
        }
        Ok(false)
    }

    pub fn contains(&self, package: &str) -> bool {
        self.packages.contains_key(package)
    }

    pub fn get_uid(&self, package: &str) -> usize {
        *self.packages.get(package).unwrap()
    }

    pub fn is_whitelist(&self, uid: usize) -> bool {
        self.whitelist.contains(&uid)
    }
}
