use crate::support::Opaque;

extern "C" {
    fn cpp_string_destroy(ptr: *const InternalCppString);
    fn cpp_string_size(ptr: *const InternalCppString) -> usize;
    fn cpp_string_data(ptr: *const InternalCppString) -> *const u8;
}

#[repr(C)]
#[derive(Debug)]
struct InternalCppString(Opaque);

pub trait Func1tr {
    fn to_str(&mut self, a: &str) -> &str;
}

pub struct CppString(*const InternalCppString);

impl CppString {
    pub fn from_raw(ptr: *const libc::c_void) -> CppString {
        CppString(ptr as *const _ as *const InternalCppString)
    }

    pub fn size(&self) -> usize {
        unsafe {
            cpp_string_size(&*self.0)
        }
    }

    pub fn data(&self) -> *const u8 {
        unsafe {
            cpp_string_data(&*self.0)
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.data(), self.size())
        }
    }

    pub fn to_str(&mut self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(self.to_bytes())
        }
    }
}

impl Drop for CppString {
    fn drop(&mut self) {
        unsafe {
            cpp_string_destroy(&*self.0);
        }
    }
}
