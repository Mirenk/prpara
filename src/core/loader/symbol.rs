use std::collections::HashMap;

use crate::core::{parasite::Address, Proc};
use crate::Result;

pub type SymHash = HashMap<String, Address>;

pub fn get_sym_hashmap(proc: Proc) -> Result<SymHash> {
    let symhash = HashMap::new();
    return Ok(symhash);
}
