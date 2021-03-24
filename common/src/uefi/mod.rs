use core::ffi::c_void;

pub type Status = u64;

// 途中にnull文字が含まれず、最後がnull文字となっている
#[repr(C)]
pub struct UTF16String([u16]);

impl UTF16String {
    pub fn from_utf16(v: &[u16]) -> Option<&Self> {
        let mut i = 0;
        while let Some(x) = v.get(i) {
            match x {
                0x0000 => {
                    if i == v.len() - 1 {
                        return Some(unsafe { Self::from_utf16_unchecked(v) });
                    } else {
                        // 途中で終端文字が出てきた
                        return None;
                    }
                }
                0x0001..=0xD7FF | 0xE000..=0xFFFF => {
                    // 2 byte
                    i += 1;
                }
                _ => {
                    // 4 byte
                    i += 2;
                }
            }
        }

        None
    }

    pub unsafe fn from_utf16_unchecked(v: &[u16]) -> &Self {
        core::mem::transmute(v)
    }

    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }
}

pub use utf16_literal::utf16 as utf16_raw;

#[macro_export]
macro_rules! utf16 {
    ($x:tt) => {{
        $crate::uefi::UTF16String::from_utf16($crate::uefi::utf16_raw!($x)).unwrap()
    }};
}

#[repr(C)]
pub struct Handle(*mut c_void);

#[repr(C)]
pub struct SystemTable {
    dummy: [u8; 52],
    console_out_handle: Handle,
    con_out: *const SimpleTextOutputProtocol,
}

impl SystemTable {
    pub fn con_out(&self) -> &SimpleTextOutputProtocol {
        unsafe { &*self.con_out }
    }
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    dummy: *mut c_void,
    output_string: extern "efiapi" fn(*const SimpleTextOutputProtocol, *const u16) -> Status,
}

impl SimpleTextOutputProtocol {
    pub fn output_string(&self, s: &UTF16String) {
        (self.output_string)(self, s.as_ptr());
    }
}
