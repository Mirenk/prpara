use std::{collections::HashMap, path::Path};

use proc_maps::get_process_maps;

use self::symbol::{get_reloc_object, set_sym_hashmap, SymHash};

use super::{
    jmp,
    parasite::{debug_rip, mmap, write_to_proc},
    Address, Pid, Proc,
};

use crate::{types::Error, Result};

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

pub fn load_shared_object(proc: Proc, filename: &Path) {
    let pid = nix::unistd::Pid::from_raw(proc.pid.try_into().unwrap());
    let symhash = &proc.symhash;
    let obj = get_reloc_object(filename, symhash).unwrap();
    let addr = unsafe { mmap(pid, obj.len()) }.unwrap();

    let _ = write_to_proc(pid, addr, obj);
    let _ = jmp(proc, addr);
    //    debug_rip(pid);
}
