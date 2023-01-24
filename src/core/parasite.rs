use nix::libc::{SYS_mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};

use super::Proc;
use crate::Result;

pub type Address = u64;

pub unsafe fn mmap(mut proc: Proc) -> Result<u64> {
    let mut regs = proc.get_regs()?.clone();

    regs.rax = SYS_mmap as u64;
    regs.rdx = 0;
    regs.rsi = 4096;
    regs.rdx = (PROT_EXEC | PROT_READ | PROT_WRITE) as u64;
    regs.rcx = (MAP_PRIVATE | MAP_ANONYMOUS) as u64;
    regs.r10 = regs.rcx;
    regs.r8 = u64::MAX;
    regs.r9 = 0;

    return proc.run_syscall(regs);
}

pub fn write_to_proc(proc: Proc, addr: Address, data: Vec<u8>) -> Result<usize> {
    return Ok(0);
}

/*
pub fn mmap (
    proc: Proc,
    addr: Option<NonZeroUsize>,
    lengh: NonZeroUsize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: RawFd,
    offset: off_t
) -> () {
    let mut regs = proc.regs;

    regs.rax = SYS_mmap as u64;
    regs.rdx = 0;
    regs.rsi = size;
}
*/
