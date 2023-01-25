use std::collections::HashMap;

use self::symbol::SymHash;

use super::{Address, Pid};

use crate::Result;

mod symbol;

pub struct Var {
    addr: Address,
    sym_hash: Option<SymHash>,
}

pub type VarHash = HashMap<String, Var>;

pub fn get_var_hash(pid: Pid) -> Result<VarHash> {
    let var_hash = HashMap::new();
    return Ok(var_hash);
}

fn get_addr(addrinfo: String) -> Result<Address> {
    return Ok(0);
}
