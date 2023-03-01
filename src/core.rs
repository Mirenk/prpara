use crate::{types::Pid, Result};

pub mod loader;
pub mod parasite;

pub struct Target {
    proc: parasite::Proc,
    loader: loader::Loader,
}

pub fn new(pid: Pid) -> Result<Target> {
    let obj = Target {
        proc: parasite::new(pid)?,
        loader: loader::new(pid)?,
    };
    Ok(obj)
}
