/*
use super::Proc;

use std::{
    ffi::c_long,
    num::NonZeroUsize,
    os::{raw::c_void, unix::raw::off_t},
};

use libc::{
    user_regs_struct, SYS_mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE,
};
use nix::sys::{
    mman::ProtFlags,
    ptrace::{self, Options},
    signal::Signal,
    wait::{waitpid, WaitStatus},
};

pub fn run_syscall(mut proc: Proc) {
    let pid = proc.pid;

    let orig_regs = proc.get_regs().unwrap();
    let rip = orig_regs.rip as *mut c_void;
    let code = 0xcc050f as *mut c_void;
    let mut regs = orig_regs;

    // debug: print rip
    println!("rip: 0x{}", format!("{:016x}", orig_regs.rip));

    let orig_code = ptrace::read(pid, rip).unwrap();
    println!("orig_code: 0x{}", format!("{:016x}", orig_code));

    // mmap syscall regs
    regs.rax = SYS_mmap as u64;
    regs.rdx = 0;
    regs.rsi = 4096;
    regs.rdx = (PROT_EXEC | PROT_READ | PROT_WRITE) as u64;
    regs.rcx = (MAP_PRIVATE | MAP_ANONYMOUS) as u64;
    regs.r10 = regs.rcx;
    regs.r8 = u64::MAX;
    regs.r9 = 0;

    ptrace::setregs(pid, regs).expect("ptrace::setregs failed.");

    unsafe {
        ptrace::write(pid, rip, code).expect("ptrace::write failed.");
    }

    ptrace::cont(pid, None).expect("ptrace::cont failed.");

    let _ = waitpid(pid, None);

    ptrace::setregs(pid, orig_regs);

    let orig_code = orig_code as *mut c_void;
    unsafe {
        ptrace::write(pid, rip, orig_code);
    }

    ptrace::cont(pid, None).expect("ptrace::cont failed.");
}
*/
