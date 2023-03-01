use std::collections::HashMap;

use crate::{
    types::{Address, Pid},
    Result,
};

pub type SymMap = HashMap<String, Address>;

pub struct Loader {
    proc_sym_map: SymMap,
}

pub fn new(pid: Pid) -> Result<Loader> {
    let obj = Loader {
        proc_sym_map: SymMap::new(),
    };
    Ok(obj)
}
