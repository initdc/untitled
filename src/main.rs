#![no_std]
#![no_main]
mod vag_buffer;
use core::panic::PanicInfo;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vag_buffer::print_something();
    loop {}
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
