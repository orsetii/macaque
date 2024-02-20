use mycelium_bitfield::bitfield;

use crate::{arch::mm::allocator::MaqAllocator, println};

use super::{
    addr::{PhysAddr, VirtAddr},
    page::Page,
    PAGE_SIZE,
};

/// SV39 page tables contain 2^9 PTE each - (8 bytes each).
#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new() -> Self {
        Self {
            entries: [PageTableEntry::new(); 512],
        }
    }

    pub fn map(&mut self, a: &mut MaqAllocator, virt: VirtAddr, phys: PhysAddr, flags: PageTableEntry, lvl: usize) {

        let vpn = virt.vpn();
        let ppn = phys.ppn();

        let mut v = &mut self.entries[vpn[2] as usize];

        // Now, we're going to traverse the page table and set the bits
	// properly. We expect the root to be valid, however we're required to
	// create anything beyond the root.
        for i in (lvl..2).rev() {
            if !v.is_valid() {

                let page = a.zalloc(1).expect("No page found");

                v.set_bits(page as u64 >> 2);
                v.set(PageTableEntry::VALID, true);
            }
            let entry = ((v.bits() & !0x3ff) << 2) as *mut PageTableEntry;
            v = unsafe { entry.add(vpn[i] as usize).as_mut().unwrap() };
        }

        let mut entry = PageTableEntry::from_bits((ppn[2] << 28) |
                    (ppn[1] << 19)|
                    (ppn[0] << 10) | flags.bits());

        entry.set(PageTableEntry::VALID, true);
        entry.set(PageTableEntry::DIRTY, true);
        entry.set(PageTableEntry::ACCESSED, true);


        v.set_bits(entry.bits());
        println!("PTE at VPN {}=>{}=>{}: {:#x?}", vpn[2], vpn[1], vpn[0], entry);
        assert_eq!(entry, self.entries[vpn[2] as usize]);
    }
}

bitfield! {
    #[derive(Eq, PartialEq)]
    pub struct PageTableEntry<u64> {
        pub const VALID: bool;
        pub const READABLE: bool;
        pub const WRITABLE: bool;
        pub const EXECUTABLE: bool;

        pub const USER_ACCESSIBLE: bool;
        /// A global mapping that exists
        /// in all address spaces
        pub const GLOBAL: bool;

        /// Indicates if the virtual page
        /// has been read, written or fetched
        /// since the last time `ACCESSED` was cleared.
        pub const ACCESSED: bool;
        /// Indicates if the virtual page has been written
        /// since the last time `DIRTY` was cleared.
        pub const DIRTY: bool;

        const _SUPERVISOR_RESERVED = 2;
        pub const PPN_1 = 9;
        pub const PPN_2 = 9;
        pub const PPN_3 = 9;

        pub const _RESERVED = 7;

        /// Reserved for use by the Svpbmt extension
        pub const PBMT = 2;
        pub const N = 1;
    }
}

impl PageTableEntry {
    /// Is this PTE mapped to a physical frame?
    pub fn is_valid(&self) -> bool {
        self.get(Self::VALID)
    }

    pub fn set_bits(&mut self, v: u64) {
        *self = Self::from_bits(v);
    }
}
