use std::{collections::HashSet, fs, path::PathBuf, process::Command, str::FromStr};

use anyhow::Result;
use app::App;
use r#enum::{Mode, V1Mode, V2Mode};
use inotify::{Inotify, WatchMask};
use lazy_static::lazy_static;
use regex::Regex;

mod app;
mod config;
pub mod r#enum;

lazy_static! {
    static ref COMPONENT_RE: Regex = Regex::new(r".*\{([^/]+)/").unwrap();
}

#[allow(non_snake_case)]
pub struct Freezer {
    app: App,
    mode: Option<Mode>,
    v2: Option<V2Mode>,
    v1: Option<V1Mode>,
    pendingHandleList: PendingHandleList,
}

struct PendingHandleList {
    list: HashSet<usize>,
}

impl PendingHandleList {
    fn new() -> Self {
        Self {
            list: HashSet::new(),
        }
    }

    fn erase(&mut self, uid: usize) {
        self.list.remove(&uid);
    }

    fn add(&mut self, uid: usize) {
        self.list.insert(uid);
    }

    fn contains(&self, uid: usize) -> bool {
        self.list.contains(&uid)
    }
}

impl Freezer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            app: App::new()?,
            mode: None,
            v2: None,
            v1: None,
            pendingHandleList: PendingHandleList::new(),
        })
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
                        if !self.app.is_whitelist(uid) {
                            cur_foreground_app.insert(uid);
                        }
                    }
                }
            }
        }
        cur_foreground_app
    }

    #[allow(non_snake_case)]
    fn check_cgroup(&mut self) -> Result<()> {
        let cgroupV2FreezerPath = PathBuf::from_str("/sys/fs/cgroup/uid_0/cgroup.freeze")?;
        let cgroupV2frozenPath = PathBuf::from_str("/sys/fs/cgroup/frozen/cgroup.freeze")?;
        let cgroupV2unfrozenPath = PathBuf::from_str("/sys/fs/cgroup/unfrozen/cgroup.freeze")?;
        let cgroupV1frozenPath = PathBuf::from_str("/dev/freezer/frozen/cgroup.procs")?;
        let cgroupV1UnfrozenPath = PathBuf::from_str("/dev/freezer/unfrozen/cgroup.procs")?;

        if cgroupV2FreezerPath.exists() {
            self.mode = Some(Mode::V2);
            self.v2 = Some(V2Mode::Uid);
            return Ok(());
        }
        if cgroupV2frozenPath.exists() && cgroupV2unfrozenPath.exists() {
            self.mode = Some(Mode::V2);
            self.v2 = Some(V2Mode::Frozen);
            return Ok(());
        }
        if cgroupV1frozenPath.exists() && cgroupV1UnfrozenPath.exists() {
            self.mode = Some(Mode::V1);
            self.v1 = Some(V1Mode::Frozen);
            return Ok(());
        }
        Ok(())
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let mut inotify = Inotify::init()?;
        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;
        let config = config::ConfigData::new()?;
        self.app.add_whitelist(config.white_list);
        self.check_cgroup()?;
        #[cfg(debug_assertions)]
        {
            log::debug!("cgroup挂载情况:{:?}", self.mode);
            log::debug!("v1挂载情况:{:?}", self.v1);
            log::debug!("v2挂载情况:{:?}", self.v2);
        }
        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            self.app.refresh_packages()?;
            let visible_app = self.get_visible_app();

            #[cfg(debug_assertions)]
            {
                log::debug!("当前顶层应用uid: {:?}", visible_app);
                log::debug!("当前pendingHandleList: {:?}", self.pendingHandleList.list);
            }

            for i in visible_app.clone() {
                self.pendingHandleList.add(i);
                if visible_app.len() == 1 {
                    self.pendingHandleList.erase(i);
                }
            }

            self.freezer();
        }
    }

    fn freezer(&mut self) {
        let visible_app = self.get_visible_app();
        let mode = match self.mode {
            Some(s) => s,
            None => {
                log::error!("无cgroup使用, 将使用SIGSTOP");
                Mode::SIGSTOP
            }
        };
        let mut freezePath = match mode {
            Mode::V2 => match self.v2 {
                Some(v2) => match v2 {
                    V2Mode::Uid => PathBuf::from_str("/sys/fs/cgroup/uid_xxx/cgroup.freeze").unwrap(),
                    V2Mode::Frozen => {
                        PathBuf::from_str("/sys/fs/cgroup/frozen/cgroup.freeze").unwrap()
                    }
                },
                None => {
                    log::error!("无法判断V2类型");
                    PathBuf::new()
                }
            },
            Mode::V1 => PathBuf::from_str("/dev/freezer/frozen/cgroup.procs").unwrap(),
            Mode::SIGSTOP => PathBuf::new(),
        };

        #[cfg(debug_assertions)]
        {
            log::debug!("{}", freezePath.display());
        }
    }
}
