#[cfg(feature = "riscv-sv39")]
pub mod sv39;
#[cfg(feature = "riscv-sv39")]
pub use sv39::*;

use crate::println;

mod constants;

pub mod allocator;

const HEAP_PAGES: u64 = 64;

pub fn initialize() {
    sv39::initialize();

    let mut allocator = allocator::MaqAllocator::new();
    println!("Created allocator: {:#x?}", allocator);

    println!("Allocating {} heap pages...", HEAP_PAGES);
    allocator.setup_heap(64)
}
