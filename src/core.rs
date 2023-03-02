use std::{fs::File, path::Path};

use crate::{
    types::{Error, Pid},
    Result,
};

pub mod loader;
pub mod parasite;

pub struct Target {
    proc: parasite::Proc,
    loader: loader::Loader,
}

impl Target {
    pub fn parasite_func(
        &mut self,
        object_path: &Path,
        old_func_sym_name: String,
        new_func_sym_name: String,
    ) -> Result<()> {
        // get file size
        let object_file = File::open(object_path).map_err(|_| Error::ElfLoadError)?;
        let file_size = object_file.metadata().unwrap().len() as usize;

        // memory alloc
        let alloc_addr = self.proc.mem_alloc(file_size)?;

        // get relocate object
        let reloc_object_buf = self.loader.get_reloc_object(object_path, alloc_addr)?;

        // write relocate object
        let _ = self.proc.write_buf(alloc_addr, reloc_object_buf)?;

        // get jump addr
        let jmp_addr = self
            .loader
            .get_address_from_proc(new_func_sym_name)
            .unwrap();

        // prepare jump buffer
        let mut jmp_buf: Vec<u8> = Vec::new();
        jmp_buf.push(0xff);
        jmp_buf.push(0x25);
        jmp_buf.push(0x00);
        jmp_buf.push(0x00);
        jmp_buf.push(0x00);
        jmp_buf.push(0x00);
        jmp_buf.extend_from_slice(&(jmp_addr).to_le_bytes());

        // write jump buffer
        let old_func_addr = self
            .loader
            .get_address_from_proc(old_func_sym_name)
            .unwrap();
        let _ = self.proc.write_buf(old_func_addr, jmp_buf);

        Ok(())
    }
}

pub fn new(pid: Pid) -> Result<Target> {
    let obj = Target {
        proc: parasite::new(pid)?,
        loader: loader::new(pid)?,
    };
    Ok(obj)
}
