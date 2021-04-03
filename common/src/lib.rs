#![no_std]
#![feature(abi_efiapi)]
#![feature(asm)]
#![feature(ptr_internals)]

mod hlt;
pub use hlt::*;
pub mod uefi;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    hlt_loop()
}
