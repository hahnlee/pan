use crate::jsi::buffer::Buffer;
use crate::support::Opaque;

extern "C" {
    fn memory_buffer__new(data: *const u8, size: usize) -> *const InternalMemoryBuffer;
    fn memory_buffer__size(ptr: *const InternalMemoryBuffer) -> usize;
}

#[repr(C)]
#[derive(Debug)]
struct InternalMemoryBuffer(Opaque);

pub struct MemoryBuffer(*const InternalMemoryBuffer);

impl MemoryBuffer {
    pub fn from_bytes(data: &[u8]) -> Self {
        unsafe { MemoryBuffer(memory_buffer__new(data.as_ptr(), data.len())) }
    }
}

impl Buffer for MemoryBuffer {
    fn size(&self) -> usize {
        unsafe { memory_buffer__size(self.0) }
    }

    fn to_ptr(&self) -> *const libc::c_void {
        self.0 as *const libc::c_void
    }
}
