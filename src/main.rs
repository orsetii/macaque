#![no_std]
#![no_main]

pub mod arch;
pub mod cpu;
pub mod drivers;
pub mod sync;
pub mod util;

#[no_mangle]
pub extern "C" fn kmain() {
    println!("Intialization Complete. Kernel Main starting...")
}
