use crate::{types::Pid, Result};

pub struct Proc {
    pid: Pid,
}

pub fn new(pid: Pid) -> Result<Proc> {
    let obj = Proc { pid };
    Ok(obj)
}
