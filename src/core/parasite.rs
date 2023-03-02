use nix::libc::{SYS_mmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};
use nix::sys::ptrace::{self, Options};
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::{libc::user_regs_struct, sys::wait::WaitStatus};

use crate::types::Error;
use crate::{
    types::{Address, Pid},
    Result,
};

pub struct Proc {
    pid: nix::unistd::Pid,
}

impl Proc {
    pub fn mem_alloc(&mut self, size: usize) -> Result<Address> {
        let orig_regs = ptrace::getregs(self.pid).map_err(|_| Error::PtraceGetRegsError)?;

        // mmap(0, size, PROT_EXEC|PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
        let mmap_regs = prepare_mmap(
            &orig_regs,
            0,
            size as u64,
            (PROT_EXEC | PROT_READ | PROT_WRITE) as u64,
            (MAP_PRIVATE | MAP_ANONYMOUS) as u64,
            u64::MAX,
            0,
        );
        self.run_syscall(mmap_regs, None)
    }
    pub fn write_buf(&mut self, addr: Address, data: Vec<u8>) -> Result<()> {
        Ok(())
    }

    pub fn run_syscall(&self, regs: user_regs_struct, stack: Option<Vec<u8>>) -> Result<u64> {
        Ok(0)
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
}

// make proc instance
pub fn new(pid: Pid) -> Result<Proc> {
    let pid = nix::unistd::Pid::from_raw(pid);

    // attach to process
    ptrace::attach(pid).map_err(|_| Error::PtraceAttachError)?;
    if let Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) = waitpid(pid, None) {
        ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD)
            .map_err(|_| Error::PtraceSetOptionError)?;

        let obj = Proc { pid };

        Ok(obj)
    } else {
        Err(Error::WaitPidError)
    }
}

// example for run_syscall()
fn prepare_mmap(
    orig_regs: &user_regs_struct,
    addr: u64,
    len: u64,
    prot: u64,
    flags: u64,
    fd: u64,
    offset: u64,
) -> user_regs_struct {
    let mut regs = orig_regs.clone();

    // set args to regs
    regs.rax = SYS_mmap as u64;
    regs.rdx = addr;
    regs.rsi = len;
    regs.rdi = prot;
    regs.rcx = flags;
    regs.r10 = flags;
    regs.r8 = fd;
    regs.r9 = offset;

    regs
}
