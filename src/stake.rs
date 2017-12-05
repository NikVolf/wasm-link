#![no_main]
#![no_std]

extern crate pwasm_std;

extern "C" {
    #[link(name = "by_hash")]
    pub fn by_hash(hash_ptr: *const u8);
}

#[no_mangle]
pub unsafe extern "C" fn add_stake(addr: *const u8, stake: *const u8) {
    by_hash(addr)
}