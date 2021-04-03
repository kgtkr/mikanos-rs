use core::{ffi::c_void, ptr::NonNull};

#[repr(C)]
pub struct TableHeader {
    dummy: [u8; 24],
}

#[repr(C)]
pub struct GUID {
    data1: u64,
    data2: u32,
    data3: u32,
    data4: [u8; 8],
}

#[repr(C)]
pub struct Handle(NonNull<c_void>);
