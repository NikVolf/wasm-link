#![no_main]
#![no_std]

extern crate pwasm_std;

#[no_mangle]
pub unsafe extern "C" fn by_hash(hash_ptr: *const u8) {
}