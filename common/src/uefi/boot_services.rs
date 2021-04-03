use core::{ffi::c_void, ptr::NonNull};

use super::{Handle, Status, TableHeader, GUID};

pub type PhysicalAddress = u64;

pub type VirtualAddress = u64;

#[repr(C)]
pub struct MemoryDescriptor {
    type_: u32,
    physical_start: PhysicalAddress,
    virtual_start: VirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

#[repr(C)]
pub struct BootServices {
    hdr: TableHeader,
    dummy1: [usize; 4],
    get_memory_map: extern "efiapi" fn(
        memory_map_size: NonNull<usize>,
        memory_map: NonNull<MemoryDescriptor>,
        map_key: NonNull<usize>,
        descriptor_size: NonNull<usize>,
        descriptor_version: NonNull<u32>,
    ) -> Status,
    dummy2: [usize; 27],
    open_protocol: extern "efiapi" fn(
        handle: Handle,
        protocol: NonNull<GUID>,
        interface: NonNull<NonNull<c_void>>,
        descriptor_size: NonNull<usize>,
        descriptor_version: NonNull<u32>,
        agent_handle: Option<Handle>,
        controller_handle: Handle,
        attributes: u32,
    ) -> Status,
}

impl BootServices {
    pub fn get_memory_map() {}
}
