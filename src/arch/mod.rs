pub mod riscv64;

pub trait Arch {
    extern "C" fn kinit();
}
