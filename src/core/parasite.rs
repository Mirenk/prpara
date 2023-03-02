use nix::libc::user_regs_struct;

use crate::{
    types::{Address, Pid},
    Result,
};

pub struct Proc {
    pid: Pid,
}

impl Proc {
    pub fn mem_alloc(&mut self, size: usize) -> Result<Address> {
        Ok(0)
    }
    pub fn write_buf(&mut self, addr: Address, data: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn run_syscall(&self, regs: user_regs_struct, stack: Option<Vec<u8>>) -> Result<u64> {
        Ok(0)
    }
}

fn prepare_mmap(orig_regs: &user_regs_struct) -> user_regs_struct {
    let mut regs = orig_regs.clone();
    regs
}

pub fn new(pid: Pid) -> Result<Proc> {
    let obj = Proc { pid };
    Ok(obj)
}
