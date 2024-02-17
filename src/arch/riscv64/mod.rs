use crate::{arch::mm::alloc::MaqAllocator, println};

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
            if let Some(ptr) = alloca.alloc_pages(64) {
                println!("Found and allocated pages at {:0p}", ptr);
                alloca.print_page_table();
                mm::init_mm(&alloca);
            } else {
                println!("Unable to find space to allocate {} pages for kernel!", 64);
                panic!();
            }
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
