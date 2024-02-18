use mycelium_bitfield::bitfield;

bitfield! {
    #[derive(PartialEq, Eq)]
    pub struct PageEntry<u8> {
        pub const TAKEN: bool;
        pub const LAST: bool;
    }
}
