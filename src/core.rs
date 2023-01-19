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
        ptrace::attach(pid).expect("ptrace::attach failed.");
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) => {
                ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD)
                    .expect("ptrace::setoptions failed.");
                let obj = Proc {
                    pid: pid,
                    regs: ptrace::getregs(pid).unwrap(),
                    syscall_regs: None,
                };
                Ok(obj)
            }
            _ => Err(Error::WaitPidError),
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
