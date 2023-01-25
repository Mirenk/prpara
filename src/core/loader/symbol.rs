use std::collections::HashMap;
use std::fs;
use std::path::Path;

use goblin::Object;

use crate::core::parasite::Address;
use crate::error::Error;
use crate::Result;

pub type SymHash = HashMap<String, Address>;

pub fn set_sym_hashmap(filename: &Path, offset: usize, symhash: &mut SymHash) -> Result<()> {
    let offset = offset as u64;
    let buffer = fs::read(filename).map_err(|_| Error::MapError)?;
    println!("filename:{}", filename.display());

    if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
        for sym in elf.syms.iter() {
            symhash.insert(
                elf.strtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value + offset,
            );
        }
        for sym in elf.dynsyms.iter() {
            symhash.insert(
                elf.dynstrtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value + offset,
            );
        }
        ()
    } else {
        return Err(Error::ElfLoadError);
    }
    return Ok(());
}

pub fn fix_sym_addr(filename: &Path, symhash: &mut SymHash) -> Result<Vec<u8>> {
    let buffer = fs::read(filename).map_err(|_| Error::MapError)?;

    if let Ok(Object::Elf(mut elf)) = Object::parse(&buffer) {
        for reloc in elf.dynrelas.iter() {
            let name = elf
                .dynstrtab
                .get_at(elf.dynsyms.get(reloc.r_sym).unwrap().st_name)
                .unwrap()
                .to_string();
            //let d_val = elf.dynamic.as_mut().unwrap().get_libraries(&elf.strtab);
            dbg!(reloc.r_sym);
            dbg!(name);
        }
    }

    return Ok(buffer);
}
