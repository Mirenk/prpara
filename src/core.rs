use std::path::Path;

use crate::{types::Pid, Result};

pub mod loader;
pub mod parasite;

pub struct Target {
    proc: parasite::Proc,
    loader: loader::Loader,
}

impl Target {
    pub fn parasite_func(
        &self,
        object_path: &Path,
        old_func_sym_name: String,
        new_func_sym_name: String,
    ) -> Result<()> {
        Ok(())
    }
}

pub fn new(pid: Pid) -> Result<Target> {
    let obj = Target {
        proc: parasite::new(pid)?,
        loader: loader::new(pid)?,
    };
    Ok(obj)
}
