use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Cursor, Read, Seek, Write};
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

fn fix_addr(buffer: &mut Vec<u8>, reloc_addr: Address, r_offset: u64) {
    println!(
        "size: 0x{:016x}, r_offset: 0x{:016x}",
        buffer.len(),
        r_offset,
    );
    let mut cur = Cursor::new(buffer);

    cur.seek(std::io::SeekFrom::Start(r_offset)).unwrap();
    cur.write(&reloc_addr.to_le_bytes()).unwrap();
    cur.seek(std::io::SeekFrom::Start(r_offset)).unwrap();
}

pub fn get_reloc_object(filename: &Path, symhash: &SymHash) -> Result<Vec<u8>> {
    let buffer = fs::read(filename).map_err(|_| Error::MapError)?;
    let mut outbuf = buffer.clone();

    if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
        // relocation .dynamic
        for reloc in elf.dynrelas.iter() {
            let name = elf
                .dynstrtab
                .get_at(elf.dynsyms.get(reloc.r_sym).unwrap().st_name)
                .unwrap()
                .to_string();
            if let Some(address) = symhash.get(&name) {
                println!("reloc: {}: 0x{:016x}", name, address);
                fix_addr(&mut outbuf, *address, reloc.r_offset);
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
                fix_addr(&mut outbuf, *address, reloc.r_offset);
            } else {
                println!("reloc: {}: notfound.", name);
                continue;
            }
        }
    }

    return Ok(outbuf);
}
