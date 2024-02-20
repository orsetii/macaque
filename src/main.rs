#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

pub mod arch;
pub mod cpu;
pub mod drivers;
pub mod sync;
pub mod util;

#[no_mangle]
pub extern "C" fn kmain() {
    println!("Intialization Complete. Kernel Main starting...")
}
