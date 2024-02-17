pub mod alloc;
pub mod page;

use core::ptr::null_mut;

use self::alloc::MaqAllocator;
use crate::{
    arch::mm::page::entry::{E_RW, E_RX, VALID},
    println,
};
use page::entry::Entry;

extern "C" {
    static TEXT_START: usize;
    static TEXT_END: usize;
    static DATA_START: usize;
    static DATA_END: usize;
    static RODATA_START: usize;
    static RODATA_END: usize;
    static BSS_START: usize;
    static BSS_END: usize;
    static KERNEL_STACK_START: usize;
    static KERNEL_STACK_END: usize;
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

const ENTRY_CNT: usize = 512;

pub static mut PAGE_TABLE: *mut Table = null_mut();
// This is the head of the allocation. We start here
// when we search for a free memory location.
static mut KMEM_HEAD: *mut AllocList = null_mut();
// In the future, we will have on-demand pages
// so, we need to keep track of our memory footprint to
// see if we actually need to allocate more.
static mut KMEM_ALLOC: usize = 0;

#[repr(usize)]
enum AllocListFlags {
    Taken = 1 << 63,
}
impl AllocListFlags {
    pub fn val(self) -> usize {
        self as usize
    }
}

struct AllocList {
    pub flags_size: usize,
}
impl AllocList {
    pub fn is_taken(&self) -> bool {
        self.flags_size & AllocListFlags::Taken.val() != 0
    }

    pub fn is_free(&self) -> bool {
        !self.is_taken()
    }

    pub fn set_taken(&mut self) {
        self.flags_size |= AllocListFlags::Taken.val();
    }

    pub fn set_free(&mut self) {
        self.flags_size &= !AllocListFlags::Taken.val();
    }

    pub fn set_size(&mut self, sz: usize) {
        let k = self.is_taken();
        self.flags_size = sz & !AllocListFlags::Taken.val();
        if k {
            self.flags_size |= AllocListFlags::Taken.val();
        }
    }

    pub fn get_size(&self) -> usize {
        self.flags_size & !AllocListFlags::Taken.val()
    }
}

pub unsafe fn init_mm(a: &MaqAllocator) {
    PAGE_TABLE = a.zalloc(1).unwrap() as *mut Table;
    KMEM_ALLOC = 64;
    let kheap_head = a.alloc_base_addr as usize;
    let total_pages = KMEM_ALLOC;

    println!("TEXT:   0x{:x} -> 0x{:x}", TEXT_START, TEXT_END);
    println!("RODATA: 0x{:x} -> 0x{:x}", RODATA_START, RODATA_END);
    println!("DATA:   0x{:x} -> 0x{:x}", DATA_START, DATA_END);
    println!("BSS:    0x{:x} -> 0x{:x}", BSS_START, BSS_END);
    println!(
        "STACK:  0x{:x} -> 0x{:x}",
        KERNEL_STACK_START, KERNEL_STACK_END
    );
    println!(
        "HEAP:   0x{:x} -> 0x{:x}",
        kheap_head,
        kheap_head + total_pages * 4096
    );

    let t = PAGE_TABLE;
    println!("Mapping heap...");
    (*t).id_map_range(
        kheap_head,
        kheap_head + total_pages * 4096,
        Entry::from(E_RW),
        a,
    );
    unsafe {
        let num_pages = alloc::total_available_pages();
        println!("Mapping heap descriptors...");
        // Map heap descriptors
        (*t).id_map_range(HEAP_START, HEAP_START + num_pages, Entry::from(E_RW), a);
        // Map executable section
        (*t).id_map_range(TEXT_START, TEXT_END, Entry::from(E_RX), a);
        // Map rodata section
        // We put the ROdata section into the text section, so they can
        // potentially overlap however, we only care that it's read
        // only.
        (*t).id_map_range(RODATA_START, RODATA_END, Entry::from(E_RX), a);
        // Map data section
        (*t).id_map_range(DATA_START, DATA_END, Entry::from(E_RW), a);
        // Map bss section
        (*t).id_map_range(BSS_START, BSS_END, Entry::from(E_RW), a);
        // Map kernel stack
        (*t).id_map_range(KERNEL_STACK_START, KERNEL_STACK_END, Entry::from(E_RW), a);
    }

    // UART
    (*t).map_range(0x1000_0000, 0x1000_0000, Entry::from(E_RW), 0, a);
}

pub struct Table {
    pub entries: [Entry; ENTRY_CNT],
}

impl Table {
    pub fn len(&self) -> usize {
        ENTRY_CNT
    }

    pub fn id_map_range(
        &mut self,
        start: usize,
        end: usize,
        flags: Entry,
        a: &alloc::MaqAllocator,
    ) {
        let mut memaddr = start & !(alloc::PAGE_SIZE - 1);
        let num_kb_pages = (MaqAllocator::align_val(end, 12) - memaddr) / alloc::PAGE_SIZE;

        // I named this num_kb_pages for future expansion when
        // I decide to allow for GiB (2^30) and 2MiB (2^21) page
        // sizes. However, the overlapping memory regions are causing
        // nightmares.
        for _ in 0..num_kb_pages {
            self.map_range(memaddr, memaddr, flags, 0, a);
            memaddr += 1 << 12;
        }
    }

    // TODO store the allocator in global state
    // so we dont have to pass it in here.
    pub fn map_range(
        &mut self,
        virt_addr: usize,
        phys_addr: usize,
        flags: Entry,
        lvl: usize,
        a: &alloc::MaqAllocator,
    ) {
        println!("Mapping {:0x} => {:0x}", phys_addr, virt_addr);
        // Ensure RWX otherwise we will leak
        // memory and always page fault
        assert!(flags.is_leaf());
        // Extract out each VPN from the virtual address
        // On the virtual address, each VPN is exactly 9 bits,
        // which is why we use the mask 0x1ff = 0b1_1111_1111 (9 bits)
        let vpn = [
            // VPN[0] = virt_addr[20:12]
            (virt_addr >> 12) & 0x1ff,
            // VPN[1] = virt_addr[29:21]
            (virt_addr >> 21) & 0x1ff,
            // VPN[2] = virt_addr[38:30]
            (virt_addr >> 30) & 0x1ff,
        ];

        // Just like the virtual address, extract the physical address
        // numbers (PPN). However, PPN[2] is different in that it stores
        // 26 bits instead of 9. Therefore, we use,
        // 0x3ff_ffff = 0b11_1111_1111_1111_1111_1111_1111 (26 bits).
        let ppn = [
            // PPN[0] = phys_addr[20:12]
            (phys_addr >> 12) & 0x1ff,
            // PPN[1] = phys_addr[29:21]
            (phys_addr >> 21) & 0x1ff,
            // PPN[2] = phys_addr[55:30]
            (phys_addr >> 30) & 0x3ff_ffff,
        ];

        // We will use this as a floating reference so that we can set
        // individual entries as we walk the table.
        let mut v = self.entries[vpn[2]];
        // Now, we're going to traverse the page table and set the bits
        // properly. We expect the root to be valid, however we're required to
        // create anything beyond the root.
        // In Rust, we create a range iterator using the .. operator.
        // The .rev() will reverse the iteration since we need to start with
        // VPN[2] The .. operator is inclusive on start but exclusive on end.
        // So, (0..2) will iterate 0 and 1.
        for i in (lvl..2).rev() {
            if !v.is_valid() {
                // Allocate a page
                // TODO: remove this unwrap, do in an `if let`
                let page = a.zalloc(1).unwrap();
                // The page is already aligned by 4,096, so store it
                // directly The page is stored in the entry shifted
                // right by 2 places.
                v.set((page as i64 >> 2) | VALID);
            }
            let entry = ((v.into_inner() & !0x3ff) << 2) as *mut Entry;
            v = unsafe { *entry.add(vpn[i]).as_mut().unwrap() };
        }

        // When we get here, we should be at VPN[0] and v should be pointing to
        // our entry.
        // The entry structure is Figure 4.18 in the RISC-V Privileged
        // Specification
        let entry = (ppn[2] << 28) as i64 |   // PPN[2] = [53:28]
			(ppn[1] << 19) as i64 |   // PPN[1] = [27:19]
			(ppn[0] << 10) as i64 |   // PPN[0] = [18:10]
			flags.into_inner() |                    // Specified bits, such as User, Read, Write, etc
            VALID;
        // Set the entry. V should be set to the correct pointer by the loop
        // above.
        v.set(entry);
    }

    pub fn unmap(&mut self, a: &MaqAllocator) {
        // Start with level 2
        for lv2 in 0..self.len() {
            let ref entry_lv2 = self.entries[lv2];
            if entry_lv2.is_valid() && entry_lv2.is_branch() {
                // This is a valid entry, so drill down and free.
                let memaddr_lv1 = (entry_lv2.into_inner() & !0x3ff) << 2;
                let table_lv1 = unsafe {
                    // Make table_lv1 a mutable reference instead of a pointer.
                    (memaddr_lv1 as *mut Table).as_mut().unwrap()
                };
                for lv1 in 0..self.len() {
                    let ref entry_lv1 = table_lv1.entries[lv1];
                    if entry_lv1.is_valid() && entry_lv1.is_branch() {
                        let memaddr_lv0 = (entry_lv1.into_inner() & !0x3ff) << 2;
                        // The next level is level 0, which
                        // cannot have branches, therefore,
                        // we free here.
                        a.free(memaddr_lv0 as *mut u8);
                    }
                }
                a.free(memaddr_lv1 as *mut u8);
            }
        }
    }

    pub fn virt_to_phys(root: &Table, vaddr: usize) -> Option<usize> {
        // Walk the page table pointed to by root
        let vpn = [
            // VPN[0] = vaddr[20:12]
            (vaddr >> 12) & 0x1ff,
            // VPN[1] = vaddr[29:21]
            (vaddr >> 21) & 0x1ff,
            // VPN[2] = vaddr[38:30]
            (vaddr >> 30) & 0x1ff,
        ];

        let mut v = &root.entries[vpn[2]];
        for i in (0..=2).rev() {
            if v.is_invalid() {
                // This is an invalid entry, page fault.
                break;
            } else if v.is_leaf() {
                // According to RISC-V, a leaf can be at any level.

                // The offset mask masks off the PPN. Each PPN is 9
                // bits and they start at bit #12. So, our formula
                // 12 + i * 9
                let off_mask = (1 << (12 + i * 9)) - 1;
                let vaddr_pgoff = vaddr & off_mask;
                let addr = ((v.into_inner() << 2) as usize) & !off_mask;
                return Some(addr | vaddr_pgoff);
            }
            // Set v to the next entry which is pointed to by this
            // entry. However, the address was shifted right by 2 places
            // when stored in the page table entry, so we shift it left
            // to get it back into place.
            let entry = ((v.into_inner() & !0x3ff) << 2) as *const Entry;
            // We do i - 1 here, however we should get None or Some() above
            // before we do 0 - 1 = -1.
            v = unsafe { entry.add(vpn[i - 1]).as_ref().unwrap() };
        }

        // If we get here, we've exhausted all valid tables and haven't
        // found a leaf.
        None
    }
}
