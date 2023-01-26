mod loader;
mod parasite;

use std::ffi::c_void;
use std::path::Path;
use std::u64;

use crate::error::Error;
use crate::Result;
use nix::libc::user_regs_struct;
use nix::sys::{
    ptrace::{self, Options},
    signal::Signal,
    wait::{waitpid, WaitStatus},
};

use self::loader::symbol::SymHash;
use self::loader::{get_var_hash, load_shared_object, set_proc_symhash, VarHash};

pub type Address = u64;

pub type Pid = u64;

pub struct Proc {
    pid: Pid,
    regs: user_regs_struct,
    symhash: SymHash,
    var_hash: VarHash,
    //    syscall_regs: Option<user_regs_struct>,
}

impl Proc {
    pub fn new(pid: Pid) -> Result<Proc> {
        let nix_pid = nix::unistd::Pid::from_raw(pid.try_into().map_err(|_| Error::PidError)?);
        ptrace::attach(nix_pid).map_err(|_| Error::PtraceAttachError)?;

        let mut symhash = SymHash::new();
        set_proc_symhash(pid, &mut symhash)?;

        for (name, addr) in symhash.iter() {
            println!("{}:{}", name, format!("{:016x}", addr))
        }

        // wait attach pid
        if let Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) = waitpid(nix_pid, None) {
            ptrace::setoptions(nix_pid, Options::PTRACE_O_TRACESYSGOOD)
                .map_err(|_| Error::PtraceSetOptionError)?;

            let regs = ptrace::getregs(nix_pid).map_err(|_| Error::PtraceGetRegsError)?;

            let obj = Proc {
                pid,
                regs,
                symhash,
                var_hash: get_var_hash(pid).map_err(|_| Error::HashError)?,
                //               syscall_regs: None,
            };

            return Ok(obj);
        } else {
            return Err(Error::WaitPidError);
        }
    }

    pub fn get_regs(&mut self) -> Result<user_regs_struct> {
        let nix_pid = nix::unistd::Pid::from_raw(self.pid.try_into().map_err(|_| Error::PidError)?);
        let regs = ptrace::getregs(nix_pid).map_err(|_| Error::PtraceGetRegsError)?;
        self.regs = regs;
        return Ok(regs);
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let pid = nix::unistd::Pid::from_raw(self.pid.try_into().unwrap());
        let _ = ptrace::detach(pid, None);
    }
}

pub fn load(proc: Proc) {
    let path = Path::new("/home/mirenk/sh365/prpara/target/debug/greet.so");
    load_shared_object(proc, path);
}
