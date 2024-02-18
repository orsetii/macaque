use crate::println;

pub mod boot;
pub mod mm;
pub mod trap;

struct RiscV64;

impl super::Arch for RiscV64 {
    #[no_mangle]
    extern "C" fn kinit() {
        println!("Walnut initializing...");
        mm::initialize();
        unsafe { core::arch::asm!("nop;nop;") }
    }
}

impl RiscV64 {
    #[no_mangle]
    extern "C" fn kinit_hart() {
        let hart_id: usize;
        unsafe {
            core::arch::asm!("csrr {}, mhartid", out(reg) hart_id);
        }
        println!("Hello from hart thread {} ", hart_id);
    }
}
