#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

use common::hlt_loop;
use common::uefi;
use common::utf16;

#[no_mangle]
pub extern "efiapi" fn efi_main(
    _image_handle: uefi::Handle,
    system_table: uefi::SystemTable,
) -> uefi::Status {
    system_table.con_out().output_string(&utf16!("hello!\0"));
    hlt_loop()
}
