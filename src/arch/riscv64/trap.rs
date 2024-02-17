#[no_mangle]
pub extern "C" fn m_trap_vector() -> ! {
    loop {}
}
