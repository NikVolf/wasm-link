#![no_main]
#![no_std]

extern crate pwasm_std;

mod structs;

use structs::Block;
use pwasm_std::hash::H256;
use pwasm_std::bigint::U256;

extern "C" {
    #[link(name = "auth")]
    pub fn by_hash(hash_ptr: &H256);
}

#[no_mangle]
pub unsafe extern "C" fn add_stake(block: &Block, addr: &H256, stake: &U256) {
    by_hash(addr)
}