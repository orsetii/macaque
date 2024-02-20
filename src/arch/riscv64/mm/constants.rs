#![allow(dead_code)]

use super::PAGE_SIZE;

extern "C" {
    static HEAP_START: u64;
    static HEAP_SIZE: u64;
    static TEXT_START: u64;
    static TEXT_END: u64;
    static DATA_START: u64;
    static DATA_END: u64;
    static RODATA_START: u64;
    static RODATA_END: u64;
    static BSS_START: u64;
    static BSS_END: u64;
    static KERNEL_STACK_START: u64;
    static KERNEL_STACK_END: u64;
}


pub fn heap_page_count() -> usize {
    // this performs a fucky hack to get
    // the page count as a constant
    unsafe {
    (HEAP_SIZE / PAGE_SIZE) as usize
    }
}

pub fn heap_start() -> u64 {
    unsafe { HEAP_START }
}

pub fn heap_size() -> u64 {
    unsafe { HEAP_SIZE }
}

pub fn text_start() -> u64 {
    unsafe { TEXT_START }
}

pub fn text_end() -> u64 {
    unsafe { TEXT_END }
}

pub fn data_start() -> u64 {
    unsafe { DATA_START }
}

pub fn data_end() -> u64 {
    unsafe { DATA_END }
}

pub fn rodata_start() -> u64 {
    unsafe { RODATA_START }
}

pub fn rodata_end() -> u64 {
    unsafe { RODATA_END }
}
pub fn bss_start() -> u64 {
    unsafe { BSS_START }
}

pub fn bss_end() -> u64 {
    unsafe { BSS_END }
}

pub fn kernel_stack_start() -> u64 {
    unsafe { KERNEL_STACK_START }
}

pub fn kernel_stack_end() -> u64 {
    unsafe { KERNEL_STACK_END }
}
