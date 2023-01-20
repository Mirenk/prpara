mod inject;
mod syscall;

use std::ffi::c_void;

use crate::error::Error;
use crate::Result;
use nix::libc::user_regs_struct;
use nix::sys::{
    ptrace::{self, Options},
    signal::Signal,
    wait::{waitpid, WaitStatus},
};

pub struct Proc {
    pid: nix::unistd::Pid,
    regs: user_regs_struct,
    //    syscall_regs: Option<user_regs_struct>,
}

impl Proc {
    pub fn new(pid: nix::unistd::Pid) -> Result<Proc> {
        ptrace::attach(pid).map_err(|_| Error::PtraceAttachError)?;

        // wait attach pid
        if let Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) = waitpid(pid, None) {
            ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD)
                .map_err(|_| Error::PtraceSetOptionError)?;

            let regs = ptrace::getregs(pid).map_err(|_| Error::PtraceGetRegsError)?;

            let obj = Proc {
                pid,
                regs,
                //               syscall_regs: None,
            };

            return Ok(obj);
        } else {
            return Err(Error::WaitPidError);
        }
    }

    pub fn get_regs(&mut self) -> Result<user_regs_struct> {
        let regs = ptrace::getregs(self.pid).map_err(|_| Error::PtraceGetRegsError)?;
        self.regs = regs;
        return Ok(regs);
    }

    unsafe fn run_syscall(&self, regs: user_regs_struct) -> Result<u64> {
        let pid = self.pid;
        let rip = self.regs.rip as *mut c_void;
        let syscall_asm = 0xcc050f as *mut c_void; // syscall; int3;

        let orig_code = ptrace::read(pid, rip).map_err(|_| Error::PtraceReadError)? as *mut c_void;
        dbg!(orig_code);

        ptrace::setregs(pid, regs).map_err(|_| Error::PtraceSetRegsError)?;
        ptrace::write(pid, rip, syscall_asm).map_err(|_| Error::PtraceWriteError)?;
        ptrace::cont(pid, None).map_err(|_| Error::PtraceContinueError)?;

        if let Ok(WaitStatus::Stopped(_, Signal::SIGTRAP)) = waitpid(pid, None) {
            let regs = ptrace::getregs(pid).map_err(|_| Error::PtraceGetRegsError)?;
            ptrace::setregs(pid, self.regs).map_err(|_| Error::PtraceSetRegsError)?;
            ptrace::write(pid, rip, orig_code).map_err(|_| Error::PtraceWriteError)?;

            return Ok(regs.rax);
        } else {
            return Err(Error::WaitPidError);
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
}

pub unsafe fn write(proc: Proc) -> Result<()> {
    let ret = syscall::mmap(proc)?;
    dbg!(format!("{:016x}", ret));
    Ok(())
}