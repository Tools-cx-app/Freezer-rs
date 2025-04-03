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
        let mut s = Self {
            pids: HashMap::new(),
            packages: HashMap::new(),
            whitelist: HashSet::new(),
        };
        let _ = s.refresh_packages();
        Ok(s)
    }

    pub fn refresh_packages(&mut self) -> Result<()> {
        let proc_dir = fs::read_dir("/proc").with_context(|| "无法读取/proc/")?;
        let mut map = HashMap::new();
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

            let status_path = Path::new("/proc").join(&pid_str).join("status");
            let status_content = match fs::read_to_string(&status_path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let process_uid = match Self::parse_uid_from_status(&status_content) {
                Some(uid) => uid,
                None => continue,
            };

            if process_uid < 10000 {
                continue;
            }

            let cmdline_path = Path::new("/proc").join(pid_str).join("cmdline");
            let cmdline = match fs::read_to_string(&cmdline_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            map.insert(cmdline.trim_matches('\0').to_string(), process_uid);
        }
        self.packages = map.clone();
        Ok(())
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

    fn parse_uid_from_status(context: &str) -> Option<usize> {
        context
            .lines()
            .find(|line| line.starts_with("Uid:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|s| s.parse::<usize>().ok())
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
