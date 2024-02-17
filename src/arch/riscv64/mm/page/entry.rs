pub const NONE: i64 = 0;
pub const VALID: i64 = 1 << 0;
pub const READ: i64 = 1 << 1;
pub const WRITE: i64 = 1 << 2;
pub const EXECUTE: i64 = 1 << 3;
pub const USER: i64 = 1 << 4;
pub const GLOBAL: i64 = 1 << 5;
pub const ACCESS: i64 = 1 << 6;
pub const DIRTY: i64 = 1 << 7;

pub const E_RW: i64 = 1 << 1 | 1 << 2;
pub const E_RX: i64 = 1 << 1 | 1 << 3;
pub const E_RWX: i64 = 1 << 1 | 1 << 2 | 1 << 3;

pub const E_USER_RW: i64 = 1 << 1 | 1 << 2 | 1 << 4;
pub const E_USER_RX: i64 = 1 << 1 | 1 << 3 | 1 << 4;
pub const E_USER_RWX: i64 = 1 << 1 | 1 << 2 | 1 << 3 | 1 << 4;

// A single entry. We're using an i64 so that
// this will sign-extend rather than zero-extend
// since RISC-V requires that the reserved sections
// take on the most significant bit.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Entry(i64);

impl Entry {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 & VALID == VALID
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// A leaf has one or more RWX bits set
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.is_user_rwx() || self.is_rwx()
    }

    #[inline]
    pub fn is_branch(&self) -> bool {
        !self.is_leaf()
    }

    #[inline]
    pub fn set(&mut self, v: i64) {
        self.0 = v
    }

    #[inline]
    pub fn is_user_rwx(&self) -> bool {
        self.0 & E_USER_RWX != 0
    }

    pub fn into_inner(&self) -> i64 {
        self.0
    }
    #[inline]
    pub fn is_rwx(&self) -> bool {
        self.0 & E_RWX != 0
    }
}

impl From<i64> for Entry {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
