use std::ffi::{c_long, c_void};

use nix::libc::user_regs_struct;
use nix::libc::{SYS_mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use nix::sys::ptrace::AddressType;
use nix::sys::{
    ptrace::{self, Options},
    signal::Signal,
    wait::{waitpid, WaitStatus},
};

use super::Proc;
use crate::types::Error;
use crate::Result;

pub type Address = u64;

unsafe fn run_syscall(pid: nix::unistd::Pid, regs: user_regs_struct) -> Result<u64> {
    let orig_regs = ptrace::getregs(pid).map_err(|_| Error::PtraceGetRegsError)?;
    let rip = regs.rip as *mut c_void;
    let syscall_asm = 0xcc050f as *mut c_void; // syscall; int3;

    let orig_code = ptrace::read(pid, rip).map_err(|_| Error::PtraceReadError)? as *mut c_void;
    dbg!(orig_code);

    ptrace::setregs(pid, regs).map_err(|_| Error::PtraceSetRegsError)?;
    ptrace::write(pid, rip, syscall_asm).map_err(|_| Error::PtraceWriteError)?;
    ptrace::cont(pid, None).map_err(|_| Error::PtraceContinueError)?;

    if let Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) = waitpid(pid, None) {
        let regs = ptrace::getregs(pid).map_err(|_| Error::PtraceGetRegsError)?;
        ptrace::setregs(pid, orig_regs).map_err(|_| Error::PtraceSetRegsError)?;
        ptrace::write(pid, rip, orig_code).map_err(|_| Error::PtraceWriteError)?;

        return Ok(regs.rax);
    } else {
        return Err(Error::WaitPidError);
    }
}

pub unsafe fn mmap(pid: nix::unistd::Pid, size: usize) -> Result<u64> {
    let mut regs = ptrace::getregs(pid).map_err(|_| Error::PtraceGetRegsError)?;

    regs.rax = SYS_mmap as u64;
    regs.rdx = 0;
    regs.rsi = size as u64;
    regs.rdx = (PROT_EXEC | PROT_READ | PROT_WRITE) as u64;
    regs.rcx = (MAP_PRIVATE | MAP_ANONYMOUS) as u64;
    regs.r10 = regs.rcx;
    regs.r8 = u64::MAX;
    regs.r9 = 0;

    return run_syscall(pid, regs);
}

pub fn write_to_proc(pid: nix::unistd::Pid, addr: Address, data: Vec<u8>) -> Result<()> {
    let word_size = 8;
    let align_size = word_size - 1;

    let align_addr = addr & !align_size;
    let align_head_size = addr - align_addr;
    let len = data.len() as u64;
    let len = align_head_size + len;
    let count: usize = (len + align_size / word_size).try_into().unwrap();

    let align_head_size = align_head_size as usize;
    let mut write_buf: Vec<u8> = ptrace::read(pid, align_addr as AddressType)
        .unwrap()
        .to_le_bytes()
        .to_vec()[0..align_head_size]
        .to_vec();
    let align_tail_size = (word_size - (len % 8)) as usize;
    let mut tail_buf: Vec<u8> = ptrace::read(pid, (align_addr + (len / word_size)) as AddressType)
        .unwrap()
        .to_le_bytes()
        .to_vec()[align_tail_size..]
        .to_vec();
    let debug = tail_buf
        .iter()
        .map(|n| format!("{:02x}", n))
        .collect::<String>();
    dbg!(debug);
    write_buf.extend_from_slice(&data);
    write_buf.append(&mut tail_buf);

    for i in (0..count).step_by(8) {
        let addr = (align_addr + i as u64) as AddressType;
        let data = &write_buf[i..(i + word_size as usize)];
        let data = u64::from_le_bytes(data.to_vec().try_into().unwrap()) as *mut c_void;
        dbg!(data);

        unsafe {
            ptrace::write(pid, addr, data);
        };
    }

    return Ok(());
}

pub fn debug_rip(pid: nix::unistd::Pid) {
    ptrace::cont(pid, None);
    waitpid(pid, None);
    let regs = ptrace::getregs(pid).unwrap();

    println!("segfalt! rip: {:016x}", regs.rip);
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
