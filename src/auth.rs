#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::hash::H256;

#[no_mangle]
pub unsafe extern "C" fn by_hash(hash_ptr: &H256) {
}