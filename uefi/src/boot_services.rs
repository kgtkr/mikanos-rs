use core::{
    ffi::c_void, hint::unreachable_unchecked, marker::PhantomData, mem::MaybeUninit, ptr::NonNull,
};

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
        memory_map: NonNull<u8>,
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

pub struct MemoryMap<'a> {
    map_key: usize,
    memory_map_size: usize,
    memory_map: NonNull<u8>,
    descriptor_size: usize,
    descriptor_version: u32,
    phantom: PhantomData<&'a ()>,
}

impl<'a> MemoryMap<'a> {
    pub fn get(&self, i: usize) -> Option<&'a MemoryDescriptor> {
        if i < self.len() {
            let ptr = (self.memory_map.as_ptr() as usize + self.descriptor_size * i)
                as *const MemoryDescriptor;

            unsafe { Some(&*ptr) }
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.memory_map_size / self.descriptor_size
    }

    pub fn map_key(&self) -> usize {
        self.map_key
    }

    pub fn iter(&self) -> MemoryMapIter<'a> {
        unsafe {
            MemoryMapIter {
                ptr: NonNull::new_unchecked(self.memory_map.as_ptr() as _),
                len: self.len(),
                descriptor_size: self.descriptor_size,
                phantom: PhantomData,
            }
        }
    }
}

pub struct MemoryMapIter<'a> {
    ptr: NonNull<MemoryDescriptor>,
    len: usize,
    descriptor_size: usize,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for MemoryMapIter<'a> {
    type Item = &'a MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                let result = &*self.ptr.as_ptr();
                self.len -= 1;
                self.ptr = NonNull::new_unchecked(
                    (self.ptr.as_ptr() as usize + self.descriptor_size) as *mut MemoryDescriptor,
                );
                Some(result)
            }
        }
    }
}

pub enum GetMemoryMapError {
    BufferTooSmall { need_size: usize },
}

impl BootServices {
    pub fn get_memory_map<'a>(
        &self,
        memory_map: &'a mut [MaybeUninit<u8>],
    ) -> Result<MemoryMap<'a>, GetMemoryMapError> {
        let mut memory_map_size = memory_map.len();
        let mut map_key = MaybeUninit::<usize>::uninit();
        let mut descriptor_size = MaybeUninit::<usize>::uninit();
        let mut descriptor_version = MaybeUninit::<u32>::uninit();

        unsafe {
            let memory_map_ptr = NonNull::new_unchecked(memory_map.as_mut_ptr() as *mut u8);

            let status = (self.get_memory_map)(
                NonNull::new_unchecked(&mut memory_map_size as *mut _),
                memory_map_ptr,
                NonNull::new_unchecked(map_key.as_mut_ptr()),
                NonNull::new_unchecked(descriptor_size.as_mut_ptr()),
                NonNull::new_unchecked(descriptor_version.as_mut_ptr()),
            );

            match status {
                Status::Success => Ok(MemoryMap {
                    map_key: map_key.assume_init(),
                    memory_map_size: memory_map_size,
                    memory_map: memory_map_ptr,
                    descriptor_size: descriptor_size.assume_init(),
                    descriptor_version: descriptor_version.assume_init(),
                    phantom: PhantomData,
                }),
                Status::BufferTooSmall => Err(GetMemoryMapError::BufferTooSmall {
                    need_size: memory_map_size,
                }),
                _ => unreachable_unchecked(),
            }
        }
    }
}
