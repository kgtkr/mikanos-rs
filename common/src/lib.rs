#![no_std]
#![feature(abi_efiapi)]
#![feature(asm)]
#![feature(ptr_internals)]

mod hlt;
pub use hlt::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    hlt_loop()
}
