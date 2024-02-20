use crate::util::Address;
use mycelium_bitfield::bitfield;

bitfield! {
    #[derive(Eq, PartialEq, PartialOrd, Ord)]
    pub struct VirtAddr<u64> {
        pub const PAGE_OFFSET = 12;
        pub const VPN_0 = 9;
        pub const VPN_1 = 9;
        pub const VPN_2 = 9;
        const _RESERVED = 24;
    }
}

impl VirtAddr {

    pub fn vpn(&self) -> [u64;3] {
        [
        self.get(Self::VPN_0),
        self.get(Self::VPN_1),
        self.get(Self::VPN_2),
        ]
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn add(&mut self, a: usize) {
        // by doing it in this weird looking way
        // we use the compilers implementation of
        // address math, instead of making it ourselves.
        unsafe { self.0 = (self.0 as *mut u8).add(a) as u64 }
    }

    pub fn sub(&mut self, a: usize) {
        // by doing it in this weird looking way
        // we use the compilers implementation of the
        // address math, instead of making it ourselves.
        unsafe { self.0 = (self.0 as *mut u8).sub(a) as u64 }
    }

    pub fn at_offset(&self, offset: usize) -> Self {
        let mut new = self.clone();
        new.add(offset);
        new
    }
}

impl Address for VirtAddr {
    //
}

bitfield! {
    #[derive(Eq, PartialEq)]
    pub struct PhysAddr<u64> {
        pub const PAGE_OFFSET = 12;
        pub const PPN_0 = 9;
        pub const PPN_1 = 9;
        pub const PPN_2 = 26;
        const _RESERVED = 8;
    }
}

impl Address for PhysAddr {
    //
}

impl PhysAddr {
    pub fn ppn(&self) -> [u64;3] {
        [
        self.get(Self::PPN_0),
        self.get(Self::PPN_1),
        self.get(Self::PPN_2),
        ]
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn add(&mut self, a: usize) {
        // by doing it in this weird looking way
        // we use the compilers implementation of
        // address math, instead of making it ourselves.
        unsafe { self.0 = (self.0 as *mut u8).add(a) as u64 }
    }

    pub fn sub(&mut self, a: usize) {
        // by doing it in this weird looking way
        // we use the compilers implementation of the
        // address math, instead of making it ourselves.
        unsafe { self.0 = (self.0 as *mut u8).sub(a) as u64 }
    }

    pub fn at_offset(&self, offset: usize) -> Self {
        let mut new = self.clone();
        new.add(offset);
        new
    }
}
