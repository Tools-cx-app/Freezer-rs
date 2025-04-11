use std::{
    collections::HashSet,
    sync::{Arc, Mutex, mpsc},
    thread, usize,
};

use anyhow::Result;
use app::App;
use inotify::WatchMask;

mod app;

pub enum FreezeMode {
    V1,
    V2,
    SIGSTOP,
}

pub enum V1 {
    Frozen,
}

pub enum V2 {
    Uid,
    Frozen,
}

pub struct Freeze {
    mode: Option<FreezeMode>,
    app: Arc<Mutex<App>>,
    PendingHandleList: PendingHandleList,
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
            mode: None,
            PendingHandleList: PendingHandleList::new(),
        }
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let (visible_app_sender, visible_app_receiver) = mpsc::channel();
        let (home_sender, home_receiver) = mpsc::channel();
        let (background_packages_sender, background_packages_receiver) = mpsc::channel();
        let app_arc = Arc::clone(&self.app);
        let home = Arc::clone(&self.app);
        let mut inotify = inotify::Inotify::init()?;

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
                locked.get_visible_app();
                visible_app_sender.send(locked.VisiblePackage.clone())?;
                background_packages_sender.send(locked.BackGroundPackages.clone())?;
                home_sender.send(locked.home_uid)?;
            }
        });

        inotify.watches().add("/dev/input", WatchMask::ACCESS)?;

        loop {
            inotify.read_events_blocking(&mut [0; 1024])?;
            log::debug!("前台{:?}", visible_app_receiver.recv());
            log::debug!("后台{:?}", background_packages_receiver.recv());
            log::debug!("桌面{:?}", home_receiver.recv());
        }
    }
}
