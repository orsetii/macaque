use mycelium_bitfield::bitfield;

use crate::println;

use super::{addr::VirtAddr, PAGE_SIZE};
use core::ops::{Index, IndexMut, Range};

bitfield! {
    #[derive(PartialEq, Eq)]
    pub struct PageState<u8> {
        pub const TAKEN: bool;
        pub const LAST: bool;
    }
}

impl PageState {
    pub fn is_last(&self) -> bool {
        self.get(Self::LAST)
    }

    pub fn is_taken(&self) -> bool {
        self.get(Self::TAKEN)
    }
    pub fn is_empty(&self) -> bool {
        !self.is_taken()
    }
    pub fn set_taken(&mut self, v: bool) {
        self.set(PageState::TAKEN, v);
    }

    pub fn set_last(&mut self, v: bool) {
        self.set(PageState::LAST, v);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PageStateList {
    /// How many total pages we can allocate
    page_capacity: usize,
    /// Starting address for the list
    base_ptr: VirtAddr,
    /// Where the page state list currently is
    cursor: VirtAddr,
}

impl PageStateList {
    pub fn new(page_capacity: usize, base_ptr: VirtAddr) -> Self {
        let mut s = Self {
            page_capacity,
            base_ptr,
            cursor: base_ptr,
        };
        s.clear_all();
        s
    }
    // TODO: this whole cursor business sucks,
    // need to refactor it
    pub fn reset_cursor(&mut self) {
        self.cursor = self.base_ptr;
    }

    // TODO FIXME This does indeed suck!
    pub fn has_contig_free_space(&mut self, requested_pages: usize) -> Option<usize> {
        for i in 0..self.page_capacity - requested_pages {
            if self[i].is_taken() {
                continue;
            }
            let mut is_contig = true;
            for j in i..self.page_capacity - requested_pages {
                if self[j].is_taken() {
                    is_contig = false;
                    break;
                }
            }
            if is_contig {
                return Some(i);
            }
        }

        None
    }
    pub fn take_range(&mut self, rng: Range<usize>) {}
    pub fn free_range(&mut self, rng: Range<usize>) {}

    pub fn pages_in_use(&self) -> usize {
        0
    }

    pub fn clear_all(&mut self) {
        for p in self {
            p.set_taken(false)
        }
    }

    pub fn index_from_ptr(&self, ptr: *const Page) -> usize {
        ptr as usize - self.base_ptr.as_usize()
    }

    fn set_cursor_to_idx(&mut self, offset: usize) {
        self.cursor = self.base_ptr.at_offset(offset);
    }
}
impl Index<usize> for PageStateList {
    type Output = PageState;

    fn index(&self, idx: usize) -> &Self::Output {
        unsafe { &*(self.base_ptr.at_offset(idx).as_u64() as *mut PageState) }
    }
}
impl IndexMut<usize> for PageStateList {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        unsafe { &mut *(self.base_ptr.at_offset(idx).as_u64() as *mut PageState) }
    }
}

impl Iterator for PageStateList {
    type Item = &'static mut PageState;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // If the next page state descriptor
        // will be beyond the total capacity, stop iterating.
        let check_mem_space = self.cursor.at_offset(1);
        // TODO: double check and test all this math
        if check_mem_space.as_usize() >= self.base_ptr.as_usize() + self.page_capacity {
            None
        } else {
            self.cursor.add(1);
            unsafe { Some(&mut *(self.cursor.as_usize() as *mut PageState)) }
        }
    }
}

#[repr(C)]
pub struct Page {
    data: [u8; PAGE_SIZE as usize],
}

#[derive(Debug, Copy, Clone)]
pub struct PageRange(pub VirtAddr, pub VirtAddr);
impl Iterator for PageRange {
    type Item = *mut Page;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            self.0.add(1);
            return Some(self.0.as_u64() as *mut Page);
        }
        None
    }
}

impl PageRange {
    pub fn new(start: u64, offset: usize) -> Self {
        let s_addr = VirtAddr::from_bits(start);
        let e_addr = s_addr.at_offset(offset);
        Self(s_addr, e_addr)
    }
}

impl core::fmt::Display for PageRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "PageEntryRange(\n {:#} to:\n {:#}\n)",
            self.0, self.1
        ))
    }
}
