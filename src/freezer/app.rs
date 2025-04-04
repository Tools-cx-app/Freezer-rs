use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PKG_REGEX: Regex =
        Regex::new(r"^(?<pkg>\S+)\s+(?<uid>\d+)\s+\d+\s+/data/data/\S+").unwrap();
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
        let pkg_list_path = Path::new("/data/system/packages.list");
        let content = fs::read_to_string(pkg_list_path)
            .with_context(|| format!("无法读取 {}", pkg_list_path.display()))?;
        let mut packages = HashMap::new();

        for line in content.lines() {
            if let Some(caps) = PKG_REGEX.captures(line) {
                let pkg = caps["pkg"].to_string();
                let uid = caps["uid"]
                    .parse::<usize>()
                    .with_context(|| format!("无效UID格式: {} in line: {}", &caps["uid"], line))?;

                packages.insert(pkg, uid);
            }
        }

        self.packages = packages;
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
