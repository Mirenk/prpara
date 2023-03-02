use std::{collections::HashMap, fs, path::Path};

use goblin::Object;

use crate::{
    types::{Address, Error, Pid},
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

// load symbol address to HashMap
pub fn load_syms(path: &Path, sym_map: &mut SymMap, offset: Address) -> Result<()> {
    let buffer = fs::read(path).map_err(|_| Error::MapError)?;
    println!("object path: {}", path.display());

    if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
        // syms section
        for sym in elf.syms.iter() {
            sym_map.insert(
                elf.strtab.get_at(sym.st_name).unwrap().to_string(), // symbol name
                sym.st_value + offset,                               // absolute address
            );
        }

        // dynsyms section
        for sym in elf.dynsyms.iter() {
            sym_map.insert(
                elf.dynstrtab.get_at(sym.st_name).unwrap().to_string(), // symbol name
                sym.st_value + offset,                                  // absolute address
            );
        }

        Ok(())
    } else {
        Err(Error::ElfLoadError)
    }
}
