use std::io::Seek;
use std::io::Write;
use std::{collections::HashMap, fs, io::Cursor, path::Path};

use goblin::Object;
use proc_maps::get_process_maps;

use crate::{
    types::{Address, Error, Pid},
    Result,
};

pub type SymMap = HashMap<String, Address>;

pub struct Loader {
    proc_sym_map: SymMap,
    parasite_sym_map: SymMap,
}

impl Loader {
    pub fn get_reloc_object(&mut self, path: &Path, offset: Address) -> Result<Vec<u8>> {
        make_relocate_object(
            path,
            &mut self.proc_sym_map,
            offset,
            Some(&mut self.parasite_sym_map),
        )
    }

    pub fn get_address_from_proc(&self, name: String) -> Option<Address> {
        self.proc_sym_map.get(&name).copied()
    }

    pub fn get_address_from_parasite(&self, name: String) -> Option<Address> {
        self.parasite_sym_map.get(&name).copied()
    }
}

// make Loader instance
pub fn new(pid: Pid) -> Result<Loader> {
    let obj = Loader {
        proc_sym_map: SymMap::new(),
        parasite_sym_map: SymMap::new(),
    };
    Ok(obj)
}

// load symbols in process
fn prepare_proc_sym_map(pid: Pid, proc_sym_map: &mut SymMap) -> Result<()> {
    // read process memory map
    let proc_memmap = get_process_maps(pid).map_err(|_| Error::MapError)?;

    // search object files and load symbols
    for memmap in proc_memmap {
        if memmap.is_read() && memmap.offset == 0 {
            if let Some(path) = memmap.filename() {
                let _ = load_syms(path, proc_sym_map, memmap.start() as u64);
            }
        };
    }

    Ok(())
}

//
// load symbol address to HashMap
//
pub fn load_syms(path: &Path, sym_map: &mut SymMap, offset: Address) -> Result<()> {
    let buffer = fs::read(path).map_err(|_| Error::MapError)?;
    println!("object path: {}", path.display());

    if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
        // syms section
        for sym in elf.syms.iter() {
            sym_map.insert(
                elf.strtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value + offset, // absolute address
            );
        }

        // dynsyms section
        for sym in elf.dynsyms.iter() {
            sym_map.insert(
                elf.dynstrtab.get_at(sym.st_name).unwrap().to_string(),
                sym.st_value + offset, // absolute address
            );
        }

        Ok(())
    } else {
        Err(Error::ElfLoadError)
    }
}

//
// make relocate object
//
pub fn make_relocate_object(
    object_path: &Path,
    proc_sym_map: &mut SymMap,
    offset: Address,
    parasite_sym_map: Option<&mut SymMap>,
) -> Result<Vec<u8>> {
    // load object symbol map
    let mut object_sym_map = SymMap::new();
    let _ = load_syms(object_path, &mut object_sym_map, offset)?;

    // make buffer
    let buffer = fs::read(object_path).map_err(|_| Error::ElfLoadError)?;
    let mut outbuf = buffer.clone();

    // relocate
    if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
        // relocation .dynamic
        for reloc in elf.dynrelas.iter() {
            let name = elf
                .dynstrtab
                .get_at(elf.dynsyms.get(reloc.r_sym).unwrap().st_name)
                .unwrap()
                .to_string();

            if let Some(address) = proc_sym_map.get(&name) {
                // from process
                println!("  reloc from process: {}: 0x{:016x}", name, address);
                fix_addr(&mut outbuf, *address, reloc.r_offset);
            } else if let Some(address) = object_sym_map.get(&name) {
                // from object
                println!("  reloc from object: {}: 0x{:016x}", name, address);
                fix_addr(&mut outbuf, *address, reloc.r_offset);
            } else {
                println!("  reloc: {}: notfound.", name);
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

            println!("relocate .plt");

            if let Some(address) = proc_sym_map.get(&name) {
                // from process
                println!("  reloc from process: {}: 0x{:016x}", name, address);
                fix_addr(&mut outbuf, *address, reloc.r_offset);
            } else if let Some(address) = object_sym_map.get(&name) {
                // from object
                println!("  reloc from object: {}: 0x{:016x}", name, address);
                fix_addr(&mut outbuf, *address, reloc.r_offset);
            } else {
                println!("  reloc: {}: notfound.", name);
                continue;
            }
        }
    }

    // add to parasite_sym_map
    if let Some(sym_map) = parasite_sym_map {
        sym_map.extend(object_sym_map);
    }

    Ok(outbuf)
}

// fix address and write to buffer
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
