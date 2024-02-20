#[cfg(feature = "riscv-sv39")]
pub mod sv39;

use core::ptr::{null, null_mut};

#[cfg(feature = "riscv-sv39")]
pub use sv39::*;

use crate::{println, arch::mm::constants::{heap_size, heap_page_count}};

use self::table::PageTable;


mod constants;

pub mod allocator;


static mut KMEM_PAGE_TABLE: *mut PageTable = null_mut();



// TODO use err here
pub fn initialize() -> core::result::Result<(), ()> {
    sv39::initialize();


    let mut allocator = allocator::MaqAllocator::new();
    unsafe {
        KMEM_PAGE_TABLE = allocator.zalloc(1).unwrap() as *mut PageTable;
    }


    println!("Capacity for {} heap pages", heap_page_count());

    // Sets the intial size of the heap.
    allocator.setup_heap(64);
    // Allocate the pages for the heap
    unsafe {
        //println!("Got Heap pointer at: {:#x?}", p);
    }


    Ok(())
}


