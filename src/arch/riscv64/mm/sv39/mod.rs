//! Defines the implementation for 39-bit Virtual Memory system - `sv39`
//!
//! The 39-bit virtual address space is divided into 4KiB pages.
//!
//! Instruction fetch addresses and load and store effective addresses are 64
//! bits, however bits 63-39 MUST be equal to bit 39.
//!
//! The 27-bit VPN is translated into a 44-bit PPN via a
//! three level page table, while the 12-bit page offset is untranslated.

use crate::println;

/// Defines address types for this
/// virtual memory system. These enable
/// us to perform various operations and
/// checks on addresses as we use them.
pub mod addr;

/// Defines types and functions
/// for the Page Table and it's entries
pub mod table;

pub mod page;

const PAGE_ORDER: u64 = 12;
pub const PAGE_SIZE: u64 = 1 << 12;

pub fn initialize() {
    // sticking sstatic hit here to test
    let addr = addr::VirtAddr::from(super::constants::text_start());
    println!("Text section address: {:#x}", addr);
}
