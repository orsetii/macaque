use crate::println;

use super::{
    addr::VirtAddr,
    constants::{heap_size, heap_start},
    page::PageEntry,
    PAGE_SIZE,
};

pub type AllocResult<'a, T> = core::result::Result<T, AllocError<'a>>;

pub enum AllocError<'a> {
    Unknown(&'a str),
}

#[derive(Debug)]
pub struct MaqAllocator {
    /// The total size the heap can expand to.
    /// The initial start size that is actually allocated
    /// is passed in `setup_heap`
    pub total_heap_bounds: PageEntryRange,
    pub current_heap_end: Option<VirtAddr>,
}

impl MaqAllocator {
    pub fn new() -> Self {
        // we take in all values for this from os compile-time
        // constants.
        Self {
            current_heap_end: None,
            total_heap_bounds: PageEntryRange(
                VirtAddr::from(heap_start()),
                VirtAddr::from(heap_start() + (MaqAllocator::num_of_pages() * PAGE_SIZE)),
            ),
        }
    }

    pub fn setup_heap(&mut self, cnt: usize) {
        self.current_heap_end = Some(self.total_heap_bounds.0.at_offset(cnt));
        println!("Current allocation range: {:#}", self.allocation_range());
    }

    fn allocation_range(&self) -> PageEntryRange {
        if let Some(end_addr) = self.current_heap_end {
            PageEntryRange(self.total_heap_bounds.0, end_addr)
        } else {
            panic!("No current heap end was set upon `allocation_range` call.")
        }
    }

    fn num_of_pages() -> u64 {
        heap_size() / PAGE_SIZE
    }
}

#[derive(Debug, Copy, Clone)]
pub struct PageEntryRange(VirtAddr, VirtAddr);
impl Iterator for PageEntryRange {
    type Item = *mut PageEntry;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 < self.1 {
            self.0.add(1);
            return Some(self.0.as_u64() as *mut PageEntry);
        }
        None
    }
}

impl core::fmt::Display for PageEntryRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "Page Entry Range:\n {:#} to:\n {:#}\n",
            self.0, self.1
        ))
    }
}
