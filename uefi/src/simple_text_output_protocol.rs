use core::{ffi::c_void, ptr::NonNull};

use super::{Status, UTF16Str};

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    dummy: NonNull<c_void>,
    output_string:
        extern "efiapi" fn(this: NonNull<SimpleTextOutputProtocol>, string: NonNull<u16>) -> Status,
}

impl SimpleTextOutputProtocol {
    pub fn output_string(&self, s: &UTF16Str) {
        (self.output_string)(self.into(), s.as_ptr());
    }
}
