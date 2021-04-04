#![no_std]
#![feature(abi_efiapi)]
#![feature(asm)]
#![feature(ptr_internals)]

mod status;
pub use status::*;

mod utf16str;
pub use utf16str::*;

mod common_types;
pub use common_types::*;

mod runtime_services;
pub use runtime_services::*;

mod boot_services;
pub use boot_services::*;

mod system_table;
pub use system_table::*;

mod simple_text_output_protocol;
pub use simple_text_output_protocol::*;
