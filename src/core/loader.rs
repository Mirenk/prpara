use std::{collections::HashMap, path::Path};

use proc_maps::get_process_maps;

use self::symbol::{fix_sym_addr, set_sym_hashmap, SymHash};

use super::{Address, Pid};

use crate::{error::Error, Result};

pub mod symbol;

pub struct Var {
    addr: Address,
    sym_hash: Option<SymHash>,
}

pub type VarHash = HashMap<String, Var>;

pub fn set_proc_symhash(pid: Pid, symhash: &mut SymHash) -> Result<()> {
    let maps = get_process_maps(pid as proc_maps::Pid).map_err(|_| Error::MapError)?;

    for map in maps {
        if map.is_read() && map.offset == 0 {
            if let Some(path) = map.filename() {
                let _ = set_sym_hashmap(path, map.start(), symhash);
                let _ = fix_sym_addr(path, symhash);
            }
        };
    }

    return Ok(());
}

pub fn get_var_hash(pid: Pid) -> Result<VarHash> {
    let var_hash = HashMap::new();
    let maps = get_process_maps(pid as proc_maps::Pid).map_err(|_| Error::MapError)?;

    return Ok(var_hash);
}

fn get_addr(addrinfo: String) -> Result<Address> {
    return Ok(0);
}
