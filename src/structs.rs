use pwasm_std;

#[repr(C)]
pub struct Block {
    parent: pwasm_std::hash::H256,
    tx_ptr: *const u8,
    tx_count: usize,
}