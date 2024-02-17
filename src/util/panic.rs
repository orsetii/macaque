use crate::println;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {:#?}", info);
    loop {}
}
