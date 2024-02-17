pub mod boot;
pub mod trap;

struct RiscV64;

impl super::Arch for RiscV64 {
    #[no_mangle]
    extern "C" fn kinit() {
        unsafe { core::arch::asm!("nop;nop;") }
    }
}

impl RiscV64 {
    #[no_mangle]
    extern "C" fn kinit_hart() {
        unsafe { core::arch::asm!("nop;nop;") }
    }
}
