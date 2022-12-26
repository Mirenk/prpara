use nix::sys::{ptrace::{self, Options}, wait::{WaitStatus, waitpid}, signal::Signal};

struct Proc {
    pid: nix::unistd::Pid
}

impl Proc {
    fn new(pid: i32) -> Result<Proc, String>{
        let obj = Proc { pid: nix::unistd::Pid::from_raw(pid)};
        ptrace::attach(obj.pid);
        match waitpid(obj.pid, None) {
            Ok(WaitStatus::Stopped(_, Signal::SIGSTOP)) => {
                ptrace::setoptions(obj.pid, Options::PTRACE_O_TRACESYSGOOD);
                Ok(obj)
            }
            _ => Err(String::from("Attach failed."))
        }
    }

}

impl Drop for Proc {
    fn drop(&mut self) {
        ptrace::detach(self.pid, None);
    }
}
