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

fn fix_addr(buffer: &mut Vec<u8>, reloc_addr: Address, sh_offset: u64, r_offset: u64) {}

pub fn get_reloc_object(filename: &Path, symhash: &mut SymHash) -> Result<Vec<u8>> {
    let buffer = fs::read(filename).map_err(|_| Error::MapError)?;

    if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
        // search section offset
        let sh_offsets: HashMap<&str, u64> = elf
            .section_headers
            .iter()
            .map(|sect_header| (&elf.shdr_strtab[sect_header.sh_name], sect_header.sh_offset))
            .collect();
        let dyn_offset = sh_offsets.get(".dynamic").unwrap();
        let plt_offset = sh_offsets.get(".plt").unwrap();

        println!(
            "dyn_offset:{:016x}, plt_offset:{:016x}",
            dyn_offset, plt_offset
        );

        // relocation .dynamic
        for reloc in elf.dynrelas.iter() {
            let name = elf
                .dynstrtab
                .get_at(elf.dynsyms.get(reloc.r_sym).unwrap().st_name)
                .unwrap()
                .to_string();
            if let Some(address) = symhash.get(&name) {
                println!("reloc: {}: 0x{:016x}", name, address);
            } else {
                println!("reloc: {}: notfound.", name);
                continue;
            }
        }

        // relocation .plt
        for reloc in elf.pltrelocs.iter() {
            let name = elf
                .dynstrtab
                .get_at(elf.dynsyms.get(reloc.r_sym).unwrap().st_name)
                .unwrap()
                .to_string();
            if let Some(address) = symhash.get(&name) {
                println!("reloc: {}: 0x{:016x}", name, address);
            } else {
                println!("reloc: {}: notfound.", name);
                continue;
            }
        }
    }

    return Ok(buffer);
}
