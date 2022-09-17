use crate::handle::Local;
use crate::support::Opaque;

extern "C" {
    fn jsi__string_buffer_new(data: *const u8, size: usize) -> *const StringBuffer;
    fn jsi__string_buffer_size(buffer: *const StringBuffer) -> usize;
    fn jsi__string_buffer_delete(buffer: *mut StringBuffer);
}

pub trait Buffer {
    fn size(&self) -> usize;
}

#[repr(C)]
#[derive(Debug)]
pub struct StringBuffer(Opaque);

impl StringBuffer {
    pub fn new<'s>(string: &str) -> Local<'s, StringBuffer> {
        unsafe { Local::from_raw(jsi__string_buffer_new(string.as_ptr(), string.len())).unwrap() }
    }
}

impl Buffer for StringBuffer {
    fn size(&self) -> usize {
        unsafe { jsi__string_buffer_size(&*self) }
    }
}

impl Drop for StringBuffer {
    fn drop(&mut self) {
        unsafe { jsi__string_buffer_delete(&mut *self) };
    }
}

#[cfg(test)]
mod tests {
    use crate::jsi::buffer::{Buffer, StringBuffer};

    #[test]
    fn create_string_buffer() {
        let buffer = StringBuffer::new("Hello World!");
        assert_eq!(buffer.size(), 12);
    }
}
