use std::os::raw::c_void;

use libc::{SYS_mmap, user_regs_struct};
use nix::sys::{ptrace::{self, Options}, wait::{WaitStatus, waitpid}, signal::Signal};

pub struct Proc {
    pid: nix::unistd::Pid,
    regs: user_regs_struct
}

impl Proc {
    pub fn new(pid: i32) -> Result<Proc, String>{
        let pid = nix::unistd::Pid::from_raw(pid);
        ptrace::attach(pid).expect("ptrace::attach failed.");
        match waitpid(pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) => {
                ptrace::setoptions(pid, Options::PTRACE_O_TRACESYSGOOD).expect("ptrace::setoptions failed.");
                let obj = Proc {
                    pid: pid,
                    regs: ptrace::getregs(pid).unwrap()
                };
                Ok(obj)
            }
            _ => Err(String::from("waitpid failed."))
        }
    }

    pub fn get_regs(&self) -> user_regs_struct {
        self.regs
    }

    pub fn set_regs(&mut self) -> Result<(), String> {
        match ptrace::getregs(self.pid) {
            Ok(regs) => {
                self.regs = regs;
                Ok(())
            }
            Err(e) => Err(String::from("set_regs failed."))
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        let _ = ptrace::detach(self.pid, None);
    }
}

/*
pub fn set_mmap_regs(
    proc: Proc,
    size: u64
) -> () {
    let mut regs = proc.regs;

    regs.rax = SYS_mmap as u64;
    regs.rdx = 0;
    regs.rsi = size;
}
*/

pub fn run_syscall(proc: Proc) {
    let pid = proc.pid;

    let orig_regs = ptrace::getregs(pid).unwrap();
    let rip = orig_regs.rip as *mut c_void;
    let code = 0xcc as *mut c_void;
    let mut regs = orig_regs;

    // debug: print rip
    println!("rip: 0x{}", format!("{:016x}", orig_regs.rip));

    let orig_code = ptrace::read(pid, rip).unwrap();
    println!("orig_code: 0x{}", format!("{:016x}", orig_code));
    //        regs.rax = 0;
    unsafe {
        ptrace::write(pid, rip, code).expect("ptrace::write failed.");
    }

    ptrace::cont(pid, None).expect("ptrace::cont failed.");
}

