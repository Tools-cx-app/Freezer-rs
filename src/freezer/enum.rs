#[derive(Debug, Clone, Copy)]
pub enum Mode {
    V1,
    V2,
    SIGSTOP,
}

#[derive(Debug)]
pub enum V1Mode {
    Frozen,
}

#[derive(Debug)]
pub enum V2Mode {
    Uid,
    Frozen,
}
