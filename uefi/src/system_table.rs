use core::{ffi::c_void, ptr::NonNull};

use super::{BootServices, Handle, RuntimeServices, SimpleTextOutputProtocol, TableHeader};

#[repr(C)]
pub struct SystemTable {
    hdr: TableHeader,
    dummy: [u8; 28],
    console_out_handle: Handle,
    con_out: NonNull<SimpleTextOutputProtocol>,
    standard_error_handle: Handle,
    std_err: NonNull<SimpleTextOutputProtocol>,
    runtime_services: NonNull<RuntimeServices>,
    boot_services: NonNull<BootServices>,
    number_of_table_entries: usize,
    configuration_table: NonNull<c_void>,
}

impl SystemTable {
    pub fn con_out(&self) -> &SimpleTextOutputProtocol {
        unsafe { self.con_out.as_ref() }
    }

    pub fn boot_services(&self) -> &BootServices {
        unsafe { self.boot_services.as_ref() }
    }
}
