use crate::{arch::mm::MaqAllocator, println};

pub mod boot;
pub mod mm;
pub mod trap;

struct RiscV64;

impl super::Arch for RiscV64 {
    #[no_mangle]
    extern "C" fn kinit() {
        println!("Walnut initializing...");
        let alloca = MaqAllocator::new();
        unsafe {
            alloca.alloc_pages(64);
            alloca.print_page_table();
            core::arch::asm!("nop;nop;")
        }
    }
}

impl RiscV64 {
    #[no_mangle]
    extern "C" fn kinit_hart() {
        unsafe { core::arch::asm!("nop;nop;") }
    }
}
