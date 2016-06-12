#[no_mangle]
pub extern "C" fn add_u32s(lhs: u32, rhs: u32) -> u32 {
    lhs + rhs
}

#[no_mangle]
pub static TEST_VALUE: u32 = 100;
