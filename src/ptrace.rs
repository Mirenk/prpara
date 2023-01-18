use std::{os::{raw::c_void, unix::raw::off_t}, ffi::c_long, num::NonZeroUsize};

use libc::{SYS_mmap, user_regs_struct, PROT_EXEC, PROT_READ, PROT_WRITE, MAP_PRIVATE, MAP_ANONYMOUS};
use nix::sys::{ptrace::{self, Options}, wait::{WaitStatus, waitpid}, signal::Signal, mman::ProtFlags};

pub struct Proc {
    pid: nix::unistd::Pid,
    regs: user_regs_struct,
    syscall_regs: Option<user_regs_struct>
}

impl Proc {
    pub fn new(pid: nix::unistd::Pid) -> Result<Proc, String>{
        ptrace::attach(pid).expect("ptrace::attach failed.");
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) => {
                ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD).expect("ptrace::setoptions failed.");
                let obj = Proc {
                    pid: pid,
                    regs: ptrace::getregs(pid).unwrap(),
                    syscall_regs: None
                };
                Ok(obj)
            }
            _ => Err(String::from("waitpid failed."))
        }
    }

    pub fn get_regs(&mut self) -> Result<user_regs_struct, String> {
        match ptrace::getregs(self.pid) {
            Ok(regs) => {
                self.regs = regs;
                Ok(regs)
            }
            Err(_) => Err(String::from("set_regs failed."))
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
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
    regs.rdx = (PROT_EXEC|PROT_READ|PROT_WRITE) as u64;
    regs.rcx = (MAP_PRIVATE|MAP_ANONYMOUS) as u64;
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

