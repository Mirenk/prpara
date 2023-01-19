mod inject;
mod syscall;

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

pub struct Proc {
    pid: nix::unistd::Pid,
    regs: user_regs_struct,
    syscall_regs: Option<user_regs_struct>,
}

impl Proc {
    pub fn new(pid: nix::unistd::Pid) -> Result<Proc, String> {
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
            _ => Err(String::from("waitpid failed.")),
        }
    }

    pub fn get_regs(&mut self) -> Result<user_regs_struct, String> {
        match ptrace::getregs(self.pid) {
            Ok(regs) => {
                self.regs = regs;
                Ok(regs)
            }
            Err(_) => Err(String::from("set_regs failed.")),
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
}
