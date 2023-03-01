use crate::types::Pid;

pub struct Proc {
    pid: Pid,
}

pub fn new(pid: Pid) -> Proc {
    Proc { pid }
}
