use core::ptr::NonNull;
// 途中にnull文字が含まれず、最後がnull文字となっている
#[repr(transparent)]
pub struct UTF16Str([u16]);

impl UTF16Str {
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

    pub fn as_ptr(&self) -> NonNull<u16> {
        unsafe { NonNull::new_unchecked(self.0.as_ptr() as _) }
    }
}

pub use utf16_literal::utf16 as utf16_raw;

#[macro_export]
macro_rules! utf16 {
    ($x:tt) => {{
        $crate::uefi::UTF16Str::from_utf16($crate::uefi::utf16_raw!($x)).unwrap()
    }};
}
