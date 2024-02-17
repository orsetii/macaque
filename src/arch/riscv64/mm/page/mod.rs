pub mod entry;

pub const PAGE_EMPTY: u8 = 0;
pub const PAGE_TAKEN: u8 = 1 << 0;
pub const PAGE_LAST: u8 = 1 << 1;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Page {
    pub flags: u8,
}

impl Page {
    pub fn is_empty(&self) -> bool {
        !self.is_taken()
    }

    pub fn is_taken(&self) -> bool {
        (self.flags & PAGE_TAKEN) == PAGE_TAKEN
    }

    pub fn is_last(&self) -> bool {
        (self.flags & PAGE_LAST) == PAGE_LAST
    }

    pub fn set_flag(&mut self, flag: u8) -> &mut Page {
        if flag == PAGE_EMPTY {
            self.flags &= !flag;
        } else {
            self.flags |= flag;
        }
        self
    }
}
