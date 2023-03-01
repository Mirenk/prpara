mod loader;
mod parasite;

use std::ffi::c_void;
use std::path::Path;
use std::u64;

use crate::types::{Address, Error, Pid};
use crate::Result;
use nix::libc::user_regs_struct;
use nix::sys::{
    ptrace::{self, Options},
    signal::Signal,
    wait::{waitpid, WaitStatus},
};

use self::loader::symbol::SymHash;
use self::loader::{get_var_hash, load_shared_object, set_proc_symhash, VarHash};
use self::parasite::write_to_proc;

pub struct Proc {
    pid: Pid,
    regs: user_regs_struct,
    symhash: SymHash,
    var_hash: VarHash,
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
    let path = Path::new("/home/mirenk/sh365/prpara/target/debug/greet2.so");
    let _ = load_shared_object(proc, path);
}

pub fn jmp(proc: Proc, jmp_addr: Address) {
    let pid = nix::unistd::Pid::from_raw(proc.pid.try_into().unwrap());
    let sym_name = String::from("greet");
    let addr = *proc.symhash.get(&sym_name).unwrap() as u64;

    let mut jmp_buf: Vec<u8> = Vec::new();
    jmp_buf.push(0xff);
    jmp_buf.push(0x25);
    jmp_buf.push(0x00);
    jmp_buf.push(0x00);
    jmp_buf.push(0x00);
    jmp_buf.push(0x00);
    jmp_buf.extend_from_slice(&(jmp_addr + 0x1119).to_le_bytes());

    let debug = jmp_buf
        .iter()
        .map(|n| format!("{:02x}", n))
        .collect::<String>();
    dbg!(debug);

    write_to_proc(pid, addr, jmp_buf);
}
