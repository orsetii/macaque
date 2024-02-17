use core::mem::size_of;

use page::{Page, PageBits};

use crate::{print, println};

pub mod alloc;
pub mod page;

extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

static mut ALLOC_START: usize = 0;
const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12;

pub struct MaqAllocator {
    base_addr: *mut page::Page,
}

impl MaqAllocator {
    pub fn new() -> Self {
        let me = Self {
            base_addr: unsafe { HEAP_START as *mut Page },
        };

        // Initialize the allocation system. There are several ways that we can
        // implement the page allocator:
        // 1. Free list (singly linked list where it starts at the first free allocation)
        // 2. Bookkeeping list (structure contains a taken and length)
        // 3. Allocate one Page structure per 4096 bytes (this is what I chose)
        // 4. Others
        // Clear all pages to make sure that they aren't accidentally taken
        for i in 0..total_available_pages() {
            unsafe {
                me.page_at(i).set_flag(PageBits::Empty);
            }
        }

        // Determine where the actual useful memory starts. This will be
        // after all Page structures. We also must align the ALLOC_START
        // to a page-boundary (PAGE_SIZE = 4096). ALLOC_START =
        // (HEAP_START + num_pages * size_of::<Page>() + PAGE_SIZE - 1)
        // & !(PAGE_SIZE - 1);
        unsafe {
            ALLOC_START = MaqAllocator::align_val(
                HEAP_START + total_available_pages() * size_of::<Page>(),
                PAGE_ORDER,
            );
        }
        me
    }

    pub unsafe fn alloc_pages(&self, cnt: usize) -> Option<*mut u8> {
        assert!(cnt > 0);
        for i in 0..(total_available_pages() - cnt) {
            //
            // TODO: we are alwasy failing this assert
            // i believe the culprit may be in `page_at` or
            // somewhere in the memory write
            // we perform with the flags.
            // will investigate!
            if self.page_at(i).is_empty() && self.has_contigious_space(i, cnt) {
                self.set_flag_for_pages(i, i + cnt - 1);

                self.page_at(i + cnt - 1)
                    .set_flag(PageBits::Taken)
                    .set_flag(PageBits::Last);

                assert!(self.page_at(i + cnt - 1).is_taken());
                assert!(self.page_at(i + cnt - 1).is_last());

                return Some(((ALLOC_START + PAGE_SIZE) * i) as *mut u8);
            }
        }
        None
    }

    fn has_contigious_space(&self, starting_offset: usize, page_cnt: usize) -> bool {
        for i in starting_offset..(starting_offset + page_cnt) {
            unsafe {
                if self.page_at(i).is_taken() {
                    return false;
                }
            }
        }
        true
    }

    // TODO use a `Range` as the param for this
    unsafe fn set_flag_for_pages(&self, starting_offset: usize, page_cnt: usize) -> bool {
        for i in starting_offset..(starting_offset + page_cnt) {
            self.page_at(i).set_flag(PageBits::Taken);
            if self.page_at(i).is_taken() {
                return false;
            }
        }
        true
    }

    #[inline]
    unsafe fn page_at<'a>(&self, offset: usize) -> &'a mut Page {
        unsafe { &mut *self.base_addr.add(offset) }
    }

    #[inline]
    const fn align_val(val: usize, order: usize) -> usize {
        // Create an alignment mask
        let mask = (1 << order) - 1;

        // Align up and return the result
        (val + mask) & !mask
    }

    pub fn print_page_table(&self) {
        unsafe {
            let num_pages = HEAP_START / PAGE_SIZE;
            let mut beg = HEAP_START as *const Page;
            let end = beg.add(num_pages);
            let alloc_beg = ALLOC_START;
            let alloc_end = ALLOC_START + num_pages * PAGE_SIZE;
            println!();
            println!(
                "PAGE ALLOCATION TABLE\nMETA: {:p} -> {:p}\nPHYS: \
					0x{:x} -> 0x{:x}",
                beg, end, alloc_beg, alloc_end
            );
            println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            let mut num = 0;
            while beg < end {
                if (*beg).is_taken() {
                    let start = beg as usize;
                    let memaddr = ALLOC_START + (start - HEAP_START) * PAGE_SIZE;
                    print!("0x{:x} => ", memaddr);
                    loop {
                        num += 1;
                        if (*beg).is_last() {
                            let end = beg as usize;
                            let memaddr =
                                ALLOC_START + (end - HEAP_START) * PAGE_SIZE + PAGE_SIZE - 1;
                            print!("0x{:x}: {:>3} page(s)", memaddr, (end - start + 1));
                            println!(".");
                            break;
                        }
                        beg = beg.add(1);
                    }
                }
                beg = beg.add(1);
            }
            println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
            println!(
                "Allocated: {:>6} pages ({:>10} bytes).",
                num,
                num * PAGE_SIZE
            );
            println!(
                "Free     : {:>6} pages ({:>10} bytes).",
                num_pages - num,
                (num_pages - num) * PAGE_SIZE
            );
            println!();
        }
    }
}

#[inline]
fn total_available_pages() -> usize {
    unsafe { HEAP_SIZE / PAGE_SIZE }
}
