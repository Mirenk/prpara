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
        println!("syms:");
        for sym in elf.syms.iter() {
            symhash.insert(
                elf.strtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value + offset,
            );
            println!(
                "  {}:{}",
                elf.strtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value
            );
        }
        println!("dynsyms:");
        for sym in elf.dynsyms.iter() {
            symhash.insert(
                elf.dynstrtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value + offset,
            );
            println!(
                "  {}:{}",
                elf.strtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value
            );
        }
        ()
    };
    return Ok(());
}
