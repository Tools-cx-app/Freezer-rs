use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use anyhow::{Context, Result};
use app::App;
use config::Config;
use freezer::Freezer;
use inotify::WatchMask;
use serde::Deserialize;

mod app;
mod config;
mod freezer;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum FreezeMode {
    V1(V1),
    V2(V2),
    SIGSTOP,
    AUTO,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum V1 {
    Frozen,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum V2 {
    Uid,
    Frozen,
}

pub struct Freeze {
    mode: FreezeMode,
    app: Arc<Mutex<App>>,
    config: Arc<Mutex<Config>>,
    PendingHandleList: PendingHandleList,
}

#[derive(Clone)]
struct PendingHandleList {
    list: HashSet<usize>,
}

impl PendingHandleList {
    fn new() -> Self {
        Self {
            list: HashSet::new(),
        }
    }

    fn add(&mut self, uid: usize) {
        self.list.insert(uid);
    }

    fn remove(&mut self, uid: usize) {
        self.list.remove(&uid);
    }
}
impl Freeze {
    pub fn new() -> Self {
        Self {
            app: Arc::new(Mutex::new(App::new().unwrap())),
            mode: FreezeMode::AUTO,
            config: Arc::new(Mutex::new(Config {
                mode: FreezeMode::AUTO,
                whitelist: HashSet::new(),
            })),
            PendingHandleList: PendingHandleList::new(),
        }
    }

    pub fn UpdateAppProcess(
        &mut self,
        BackGroundPackages: HashMap<String, usize>,
        VisiblePackage: HashMap<String, usize>,
    ) {
        for (BackGroundPackage, BackGroundUid) in BackGroundPackages {
            for (_, VisiblePackageUid) in VisiblePackage.clone() {
                if BackGroundUid == VisiblePackageUid {
                    self.PendingHandleList.remove(VisiblePackageUid);
                } else {
                    self.PendingHandleList.add(BackGroundUid);
                }
            }
        }
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let (visible_app_sender, visible_app_receiver) = mpsc::channel();
        let (home_sender, home_receiver) = mpsc::channel();
        let (background_packages_sender, background_packages_receiver) = mpsc::channel();
        let (config_sender, config_receiver) = mpsc::channel();
        let app_arc = Arc::clone(&self.app);
        let config_arc = Arc::clone(&self.config);
        let mut inotify = inotify::Inotify::init()?;
        let mut freezer = Freezer::new();

        thread::spawn(move || -> Result<()> {
            let mut locked = config_arc.lock().unwrap();
            let mut inotify = inotify::Inotify::init()?;

            inotify
                .watches()
                .add("/data/freezer.toml", WatchMask::ACCESS)?;

            locked.load_config().context("无法获取配置文件")?;
            log::debug!("配置文件:{:?}", locked);
            config_sender.send((locked.mode, locked.whitelist.clone()))?;

            loop {
                inotify
                    .read_events_blocking(&mut [0; 1024])
                    .context("无法read")?;
                locked.load_config().context("无法获取配置文件")?;
                config_sender.send((locked.mode, locked.whitelist.clone()))?;
            }
        });

        thread::spawn(move || -> Result<()> {
            let mut locked = app_arc.lock().unwrap();
            let mut inotify = inotify::Inotify::init()?;

            inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

            loop {
                /*
                #[cfg(debug_assertions)]
                {
                    log::debug!("{:?}", locked.get_visible_app());
                }*/
                inotify.read_events_blocking(&mut [0; 1024])?;
                locked.ReflashPackages();
                visible_app_sender.send(locked.VisiblePackage.clone())?;
                background_packages_sender.send(locked.BackGroundPackages.clone())?;
                home_sender.send(locked.home_uid)?;
            }
        });

        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

        if let Ok(c) = config_receiver.recv() {
            let (mode, _) = c;
            freezer.SetFreezerMode(mode);
        }

        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            if let Ok(BackGroundPackages) = background_packages_receiver.recv() {
                if let Ok(VisiblePackage) = visible_app_receiver.recv() {
                    self.UpdateAppProcess(BackGroundPackages, VisiblePackage);
                }
            }
            if let Err(e) = config_receiver.recv() {
                log::error!("配置文件读取失败{e}");
            }
            #[cfg(debug_assertions)]
            {
                for i in self.PendingHandleList.list.clone() {
                    log::debug!("{i}列表pids{:?}", App::GetPids(i));
                }
                log::debug!("PendingHandleList列表{:?}", self.PendingHandleList.list);
                log::debug!("前台{:?}", visible_app_receiver.recv());
                log::debug!("后台{:?}", background_packages_receiver.recv());
                log::debug!("桌面{:?}", home_receiver.recv());
                log::debug!("当前配置文件:{:?}", config_receiver.recv());
            }
        }
    }
}
