use std::path::PathBuf;

use super::{FreezeMode, PendingHandleList};
use lazy_static::lazy_static;

lazy_static! {
    static ref cgroupV2FreezerCheckPath: PathBuf =
        PathBuf::from("/sys/fs/cgroup/uid_0/cgroup.freeze");
    static ref cgroupV2frozenCheckPath: PathBuf =
        PathBuf::from("/sys/fs/cgroup/frozen/cgroup.freeze");
    static ref cgroupV2unfrozenCheckPath: PathBuf =
        PathBuf::from("/sys/fs/cgroup/unfrozen/cgroup.freeze");
    static ref cpusetEventPath: PathBuf = PathBuf::from("/dev/cpuset/top-app");
    static ref cgroupV1frozenCheckPath: PathBuf = PathBuf::from("/dev/freezer/frozen/cgroup.procs");
    static ref cgroupV1unfrozenCheckPath: PathBuf =
        PathBuf::from("/dev/freezer/unfrozen/cgroup.procs");
    static ref cgroupV2FrozenPath: PathBuf = PathBuf::from("/sys/fs/cgroup/frozen/cgroup.procs");
    static ref cgroupV2UnfrozenPath: PathBuf =
        PathBuf::from("/sys/fs/cgroup/unfrozen/cgroup.procs");
}
pub struct Freezer {
    PendingHandleList: PendingHandleList,
    mode: FreezeMode,
}

impl Freezer {
    pub fn new(PendingHandleList: PendingHandleList) -> Self {
        Self {
            PendingHandleList,
            mode: FreezeMode::AUTO,
        }
    }

    pub fn SetFreezerMode(&mut self, mode: FreezeMode) {
        match mode {
            FreezeMode::V1(super::V1::Frozen) => {
                if cgroupV1frozenCheckPath.exists() && cgroupV1unfrozenCheckPath.exists() {
                    self.mode = FreezeMode::V1(super::V1::Frozen);
                }
            }
            FreezeMode::V2(super::V2::Frozen) => {
                if cgroupV2FrozenPath.exists() && cgroupV2UnfrozenPath.exists() {
                    self.mode = FreezeMode::V2(super::V2::Frozen);
                }
            }
            FreezeMode::V2(super::V2::Uid) => {
                if cgroupV2FreezerCheckPath.exists() {
                    self.mode = FreezeMode::V2(super::V2::Uid);
                }
            }
            FreezeMode::SIGSTOP => self.mode = FreezeMode::SIGSTOP,
            FreezeMode::AUTO => {
                if cgroupV2FrozenPath.exists() && cgroupV2UnfrozenPath.exists() {
                    self.mode = FreezeMode::V2(super::V2::Frozen);
                } else if cgroupV2FreezerCheckPath.exists() {
                    self.mode = FreezeMode::V2(super::V2::Uid);
                } else if cgroupV1frozenCheckPath.exists() && cgroupV1unfrozenCheckPath.exists() {
                    self.mode = FreezeMode::V1(super::V1::Frozen);
                } else {
                    self.mode = FreezeMode::SIGSTOP;
                }
            }
        }
        #[cfg(debug_assertions)]
        {
            log::debug!("当前模式: {:?}", self.mode);
        }
    }
}
