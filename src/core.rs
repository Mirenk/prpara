mod inject;
mod syscall;

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
    syscall_regs: Option<user_regs_struct>,
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
                syscall_regs: None,
            };

            return Ok(obj);
        } else {
            return Err(Error::WaitPidError);
        }
    }

    pub fn get_regs(&mut self) -> Result<user_regs_struct> {
        match ptrace::getregs(self.pid) {
            Ok(regs) => {
                self.regs = regs;
                Ok(regs)
            }
            Err(_) => Err(Error::PtraceGetRegsError),
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
}
