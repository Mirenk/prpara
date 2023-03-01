use std::{collections::HashMap, path::Path};

use crate::{
    types::{Address, Pid},
    Result,
};

pub type SymMap = HashMap<String, Address>;

pub struct Loader {
    proc_sym_map: SymMap,
}

impl Loader {
    pub fn get_reloc_object(&self, path: &Path) {}
}

pub fn new(pid: Pid) -> Result<Loader> {
    let obj = Loader {
        proc_sym_map: SymMap::new(),
    };
    Ok(obj)
}

fn prpare_proc_sym_map(pid: Pid, proc_sym_map: &mut SymMap) -> Result<()> {
    Ok(())
}

pub fn load_syms(path: &Path, sym_map: &mut SymMap) -> Result<()> {
    Ok(())
}
