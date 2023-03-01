use std::collections::HashMap;

use crate::types::{Address, Pid};

pub type SymMap = HashMap<String, Address>;

pub struct Loader {
    proc_sym_map: SymMap,
}

pub fn new(pid: Pid) -> Loader {
    Loader {
        proc_sym_map: SymMap::new(),
    }
}
