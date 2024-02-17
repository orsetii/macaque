#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PageBits {
    Empty = 0,
    Taken = 1,
    Last = 2,
}
impl PageBits {
    pub fn v(self) -> u8 {
        self as u8
    }
}

pub struct Page {
    flags: u8,
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.flags == PageBits::Empty.v()
    }

    pub fn is_taken(&self) -> bool {
        self.flags == PageBits::Taken.v()
    }

    pub fn is_last(&self) -> bool {
        self.flags == PageBits::Last.v()
    }

    pub fn set_flag(&mut self, flag: PageBits) -> &mut Page {
        self.flags |= flag.v();
        self
    }
}
