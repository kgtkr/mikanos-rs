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
    get_memory_map: unsafe extern "efiapi" fn(
        memory_map_size: NonNull<usize>,
        memory_map: NonNull<[MaybeUninit<u8>]>,
        map_key: NonNull<MaybeUninit<usize>>,
        descriptor_size: NonNull<MaybeUninit<usize>>,
        descriptor_version: NonNull<MaybeUninit<u32>>,
    ) -> Status,
    dummy2: [usize; 27],
    open_protocol: unsafe extern "efiapi" fn(
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
    memory_map: NonNull<MaybeUninit<MemoryDescriptor>>,
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
        MemoryMapIter {
            ptr: self.memory_map,
            len: self.len(),
            descriptor_size: self.descriptor_size,
            phantom: PhantomData,
        }
    }
}

pub struct MemoryMapIter<'a> {
    ptr: NonNull<MaybeUninit<MemoryDescriptor>>,
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
                let result = &*(self.ptr.as_ptr() as *const MemoryDescriptor);
                self.len -= 1;
                self.ptr = NonNull::new_unchecked(
                    (self.ptr.as_ptr() as usize + self.descriptor_size) as *mut _,
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
            let status = (self.get_memory_map)(
                (&mut memory_map_size).into(),
                memory_map.into(),
                (&mut map_key).into(),
                (&mut descriptor_size).into(),
                (&mut descriptor_version).into(),
            );

            match status {
                Status::Success => Ok(MemoryMap {
                    map_key: map_key.assume_init(),
                    memory_map_size: memory_map_size,
                    memory_map: NonNull::<_>::from(memory_map).cast::<_>(),
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
