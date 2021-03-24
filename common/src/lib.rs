#![no_std]
#![feature(abi_efiapi)]
#![feature(asm)]

mod hlt;
pub use hlt::*;

pub mod uefi;