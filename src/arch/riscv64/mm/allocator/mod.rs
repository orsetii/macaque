use core::ops::Range;

use mycelium_bitfield::bitfield;

use crate::println;

use super::{
    addr::VirtAddr,
    constants::{heap_page_count, heap_size, heap_start},
    page::{Page, PageRange, PageStateList},
    PAGE_SIZE,
};

pub type AllocResult<'a, T> = core::result::Result<T, AllocError<'a>>;

pub enum AllocError<'a> {
    Unknown(&'a str),
}

const ALLOCATION_LIST_INITIAL_SIZE: usize = 512;

#[derive(Debug)]
pub struct MaqAllocator {
    /// The total size the heap can expand to.
    /// The initial start size that is actually allocated
    /// is passed in `setup_heap`
    pub total_heap_bounds: PageRange,
    pub current_heap_end: Option<VirtAddr>,
    allocation_list: AllocationList<ALLOCATION_LIST_INITIAL_SIZE>,
    page_state_list: PageStateList,
}

impl MaqAllocator {
    pub fn new() -> Self {
        // we take in all values for this from os compile-time
        // constants.
        Self {
            current_heap_end: None,
            total_heap_bounds: PageRange(
                VirtAddr::from(heap_start()),
                VirtAddr::from(heap_start() + heap_size()),
            ),
            allocation_list: AllocationList {
                values: [AllocState::new(); ALLOCATION_LIST_INITIAL_SIZE],
                len: ALLOCATION_LIST_INITIAL_SIZE,
            },
            page_state_list: PageStateList::new(heap_page_count(), VirtAddr::from(heap_start())),
        }
    }

    pub fn alloc_pages(&mut self, pages: usize) -> Option<*const Page> {
            if let Some(idx) = self.page_state_list.has_contig_free_space(pages) {
                println!("Setting pages {}..{} to taken", idx, idx + pages);
                for i in idx..idx + pages {
                    self.page_state_list[i].set_taken(true);
                }
                self.page_state_list[idx + pages].set_last(true);
                self.page_by_idx(idx)
            } else {
                None
            }
    }

    pub fn free(&mut self, ptr: *const Page) {
        let mut i = self.page_state_list.index_from_ptr(ptr);
        while !self.page_state_list[i].is_last() {
            self.page_state_list[i].set_taken(false);
            i += 1;
        }
        self.page_state_list[i].set_taken(false);
        self.page_state_list[i].set_last(false);
    }

    pub fn zalloc(&mut self, pages: usize) -> Option<*const Page> {
        let p = self.alloc_pages(pages);

        if let Some(ptr) = p {

            let sz = (PAGE_SIZE as usize * pages ) / 8;
            let lg_ptr = ptr as *mut u64;

            for i in 0..sz {
                unsafe {
                    (*lg_ptr.add(i)) = 0;
                }
            }

        }

        p
    }

    pub fn setup_heap(&mut self, initial_heap_allocation: usize) {
        let r = self.alloc_pages(initial_heap_allocation);
        println!("Page allocated : {:?}", r);
        self.free(r.unwrap());
        self.current_heap_end = Some(self.total_heap_bounds.0.at_offset(initial_heap_allocation));
        println!("Current allocation range: {:#0x?}", self.allocation_range());
        println!("Total range: {:#0x?}", self.total_heap_bounds);
    }

    fn allocation_range(&self) -> PageRange {
        if let Some(end_addr) = self.current_heap_end {
            assert!(end_addr < self.total_heap_bounds.1);
            PageRange(self.heap_start(), end_addr)
        } else {
            unreachable!("No current heap end was set upon `allocation_range` call.")
        }
    }

    pub fn page_by_idx(&self, idx: usize) -> Option<*const Page> {
        println!("page_by_idx: {:#x?}", self.page_state_list[1]);
        let p_addr = self.heap_start().at_offset(idx * PAGE_SIZE as usize);
        if p_addr < self.heap_end() {
            Some(p_addr.as_u64() as *const Page)
        } else {
            None
        }
    }

    pub fn page_by_idx_mut(&self, idx: usize) -> Option<*mut Page> {
        let p_addr = self.heap_start().at_offset(idx * PAGE_SIZE as usize);
        if p_addr < self.heap_end() {
            Some(p_addr.as_u64() as *mut Page)
        } else {
            None
        }
    }


    #[inline]
    fn heap_start(&self) -> VirtAddr {
        self.total_heap_bounds.0
    }

    #[inline]
    fn heap_end(&self) -> VirtAddr {
        self.total_heap_bounds.1
    }
}

fn num_of_pages() -> u64 {
    heap_size() / PAGE_SIZE
}

bitfield! {
    pub struct AllocState<u64> {
        pub const SIZE = 63;
        pub const TAKEN: bool;
    }
}

impl AllocState {
    pub fn is_taken(&self) -> bool {
        self.get(Self::TAKEN)
    }

    pub fn is_empty(&self) -> bool {
        !self.is_taken()
    }

    pub fn set_taken(&mut self, v: bool) {
        self.set(AllocState::TAKEN, v);
    }
}

// TODO impl a push fn so we can expand
#[derive(Debug)]
pub struct AllocationList<const N: usize> {
    values: [AllocState; N],
    len: usize,
}

impl<const N: usize> AllocationList<N> {
    pub fn take_range(&mut self, rng: Range<usize>) {
        self.values[rng]
            .iter_mut()
            .for_each(|pg| pg.set_taken(true));

        // Set the last
    }
    pub fn free_range(&mut self, rng: Range<usize>) {
        self.values[rng]
            .iter_mut()
            .for_each(|pg| pg.set_taken(false));
    }

    pub fn pages_in_use(&self) -> usize {
        self.values.iter().filter(|pg| pg.is_taken()).count()
    }
}
