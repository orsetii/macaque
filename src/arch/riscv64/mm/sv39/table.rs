use mycelium_bitfield::bitfield;

/// SV39 page tables contain 2^9 PTE each - (8 bytes each).
pub struct PageTable {
    entries: [PageTableEntry; 512],
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
