use crate::types::Pid;

pub mod loader;
pub mod parasite;

pub struct Target {
    proc: parasite::Proc,
    loader: loader::Loader,
}

pub fn new(pid: Pid) -> Target {
    Target {
        proc: parasite::new(pid),
        loader: loader::new(pid),
    }
}
