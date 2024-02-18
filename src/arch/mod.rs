#[cfg(all(target_arch = "riscv64", feature = "riscv"))]
pub mod riscv64;

#[cfg(all(target_arch = "riscv64", feature = "riscv"))]
pub use riscv64::*;

pub trait Arch {
    extern "C" fn kinit();
}
