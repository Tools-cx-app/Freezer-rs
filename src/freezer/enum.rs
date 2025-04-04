#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Mode {
    V1,
    V2,
    SIGSTOP,
}

#[derive(Debug, Clone, Copy)]
pub enum V1Mode {
    Frozen,
}

#[derive(Debug, Clone, Copy)]
pub enum V2Mode {
    Uid,
    Frozen,
}
