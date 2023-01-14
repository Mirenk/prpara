use std::os::raw::c_void;

use nix::sys::{ptrace::{self, Options}, wait::{WaitStatus, waitpid}, signal::Signal};

pub struct Proc {
    pid: nix::unistd::Pid
}

impl Proc {
    pub fn new(pid: i32) -> Result<Proc, String>{
        let obj = Proc { pid: nix::unistd::Pid::from_raw(pid)};
        ptrace::attach(obj.pid).expect("ptrace::attach failed.");
        match waitpid(obj.pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) => {
                ptrace::setoptions(obj.pid, Options::PTRACE_O_TRACESYSGOOD).expect("ptrace::setoptions failed.");
                Ok(obj)
            }
            _ => Err(String::from("waitpid failed."))
        }
    }

    pub fn inject(&self) {
        let orig_regs = ptrace::getregs(self.pid).unwrap();
        let rip = orig_regs.rip as *mut c_void;
        let mut regs = orig_regs;

        // debug: print rip
        println!("rip: 0x{}", format!("{:016x}", orig_regs.rip));

//        let orig_code = ptrace::read(self.pid, rip).unwrap();
//        regs.rax = 0;
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        ptrace::detach(self.pid, None).expect("ptrace::detach failed.");
    }
}
