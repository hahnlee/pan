use crate::handle::Local;
use crate::support::Opaque;
use std::ffi::CString;

extern "C" {
    fn jsi__stringBuffer_New(data: *const libc::c_char) -> *const StringBuffer;
    fn jsi__stringBuffer_size(buffer: *const StringBuffer) -> libc::size_t;
    fn jsi__stringBuffer_delete(buffer: *mut StringBuffer);
}

pub trait Buffer {
    fn size(&self) -> libc::size_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct StringBuffer(Opaque);

impl StringBuffer {
    pub fn new<'s>(s: &str) -> Local<'s, StringBuffer> {
        let data = CString::new(s).unwrap();
        unsafe { Local::from_raw(jsi__stringBuffer_New(data.as_ptr())).unwrap() }
    }
}

impl Buffer for StringBuffer {
    fn size(&self) -> libc::size_t {
        unsafe { jsi__stringBuffer_size(&*self) }
    }
}

impl Drop for StringBuffer {
    fn drop(&mut self) {
        unsafe { jsi__stringBuffer_delete(&mut *self) };
    }
}
