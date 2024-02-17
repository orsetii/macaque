#[no_mangle]
pub extern "C" fn m_trap_vector() -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn s_trap_vector() -> ! {
    loop {}
}
