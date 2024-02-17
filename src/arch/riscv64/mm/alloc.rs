use core::mem::size_of;

use crate::{
    arch::mm::page::{PAGE_LAST, PAGE_TAKEN},
    print, println,
};

use super::{
    page::{Page, PAGE_EMPTY},
    HEAP_SIZE, HEAP_START,
};

static mut ALLOC_START: usize = 0;
const PAGE_ORDER: usize = 12;
pub const PAGE_SIZE: usize = 1 << 12;

pub struct MaqAllocator {
    pub base_addr: *mut Page,
    pub alloc_base_addr: *mut Page,
}

impl MaqAllocator {
    pub fn new() -> Self {
        let me = Self {
            base_addr: unsafe { HEAP_START as *mut Page },
            // Determine where the actual useful memory starts. This will be
            // after all Page structures. We also must align the self.alloc_base_addr
            // to a page-boundary (PAGE_SIZE = 4096). self.alloc_base_addr =
            // (HEAP_START + num_pages * size_of::<Page>() + PAGE_SIZE - 1)
            // & !(PAGE_SIZE - 1);
            alloc_base_addr: unsafe {
                MaqAllocator::align_val(
                    HEAP_START + total_available_pages() * size_of::<Page>(),
                    PAGE_ORDER,
                ) as *mut Page
            },
        };

        // Clear all pages to make sure that they aren't accidentally taken
        for i in 0..total_available_pages() {
            unsafe {
                me.page_at(i).set_flag(PAGE_EMPTY);
            }
        }

        me
    }

    /// Allocates `cnt` pages, and zeroes the entire page.
    pub fn zalloc(&self, cnt: usize) -> Option<*mut Page> {
        unsafe {
            if let Some(ptr) = self.alloc_pages(cnt) {
                let total_sz = (PAGE_SIZE * cnt) / 16;
                let big_ptr = ptr as *mut u128;
                for i in 0..total_sz {
                    (*big_ptr.add(i)) = 0;
                }

                Some(ptr)
            } else {
                None
            }
        }
    }

    pub unsafe fn alloc_pages(&self, cnt: usize) -> Option<*mut Page> {
        assert!(cnt > 0);
        println!("Allocating {} pages", cnt);
        for i in 0..(total_available_pages() - cnt) {
            if self.page_at(i).is_empty() && self.has_contigious_space(i, cnt) {
                self.set_flag_for_pages(i, i + cnt - 1);

                self.page_at(i + cnt - 1)
                    .set_flag(PAGE_TAKEN)
                    .set_flag(PAGE_LAST);

                assert!(self.page_at(i + cnt - 1).is_taken());
                assert!(self.page_at(i + cnt - 1).is_last());

                return Some((self.alloc_base_addr.add(PAGE_SIZE * i)) as *mut Page);
            }
        }
        None
    }

    // todo use errors/result here
    pub fn free(&self, ptr: *mut u8) {
        assert!(!ptr.is_null());
        unsafe {
            let addr = HEAP_START + (ptr as usize - self.alloc_base_addr as usize) / PAGE_SIZE;
            assert!(addr >= HEAP_START && addr < HEAP_START + HEAP_SIZE);
            // TODO test this using base_addr?
            // instead of calculations above,
            // then we can use `page_at`

            let mut p = addr as *mut Page;
            while (*p).is_taken() && !(*p).is_last() {
                (*p).set_flag(PAGE_EMPTY);
                p = p.add(1);
            }
            // If the following assertion fails, it is most likely
            // caused by a double-free.
            assert!(
                (*p).is_last() == true,
                "Possible double-free detected! (Not taken found \
					before last)"
            );
            // If we get here, we've taken care of all previous pages and
            // we are on the last page.
            (*p).set_flag(PAGE_EMPTY);
        }
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
    unsafe fn set_flag_for_pages(&self, starting_offset: usize, page_cnt: usize) {
        for i in starting_offset..(starting_offset + page_cnt) {
            self.page_at(i).set_flag(PAGE_TAKEN);
        }
    }

    fn get_page_for_ptr<'a>(&self, ptr: *mut u8) -> &'a mut Page {
        todo!();
    }

    #[inline]
    unsafe fn page_at<'a>(&self, offset: usize) -> &'a mut Page {
        unsafe { &mut *self.alloc_base_addr.add(offset) }
    }

    #[inline]
    pub const fn align_val(val: usize, order: usize) -> usize {
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
            let alloc_beg = self.alloc_base_addr as usize;
            let alloc_end = self.alloc_base_addr as usize + num_pages * PAGE_SIZE;
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
                    let memaddr = self.alloc_base_addr as usize + (start - HEAP_START) * PAGE_SIZE;
                    print!("0x{:x} => ", memaddr);
                    loop {
                        num += 1;
                        if (*beg).is_last() {
                            let end = beg as usize;
                            let memaddr = self.alloc_base_addr as usize
                                + (end - HEAP_START) * PAGE_SIZE
                                + PAGE_SIZE
                                - 1;
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
pub fn total_available_pages() -> usize {
    unsafe { HEAP_SIZE / PAGE_SIZE }
}
