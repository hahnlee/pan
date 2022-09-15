use crate::handle::Local;
use crate::jsi::buffer::Buffer;
use crate::jsi::object::Object;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

use std::ffi::CString;
use std::ops::Deref;

extern "C" {
    fn hermes__make_hermes_runtime() -> *mut HermesRuntime;
    fn hermes__runtime_is_hermes_bytecode(data: *const u8, len: usize) -> bool;
    fn hermes__runtime_is_inspectable(runtime: *const HermesRuntime) -> bool;
    fn hermes__runtime_get_bytecode_version() -> u32;
    fn hermes__runtime_evaluate_javascript(
        runtime: *const HermesRuntime,
        buffer: *const libc::c_void,
        source_url: *const libc::c_char,
    ) -> *const Value;
    fn hermes__runtime_global(runtime: *const HermesRuntime) -> *const Object;
    fn hermes__runtime_delete(runtime: *const HermesRuntime);
}

#[repr(C)]
#[derive(Debug)]
pub struct HermesRuntime(Opaque);

impl HermesRuntime {
    pub fn is_hermes_bytecode(data: &[u8]) -> bool {
        unsafe { hermes__runtime_is_hermes_bytecode(data.as_ptr(), data.len()) }
    }

    pub fn get_bytecode_version() -> u32 {
        unsafe { hermes__runtime_get_bytecode_version() }
    }

    pub fn new<'s>() -> Local<'s, HermesRuntime> {
        unsafe { Local::from_raw(hermes__make_hermes_runtime()).unwrap() }
    }

    pub fn from_raw<'s>(ptr: *const HermesRuntime) -> Local<'s, HermesRuntime> {
        unsafe { Local::from_raw(ptr).unwrap() }
    }

    pub fn is_inspectable(&self) -> bool {
        unsafe { hermes__runtime_is_inspectable(&*self) }
    }

    pub fn global(&self) -> Local<'_, Object> {
        unsafe { Object::from_raw(hermes__runtime_global(&*self)) }
    }
}

impl Runtime for HermesRuntime {
    fn evaluate_javascript<T: Buffer>(&self, buffer: &T, source_url: &str) -> Local<'_, Value> {
        let source_url = CString::new(source_url).unwrap();

        unsafe {
            Local::from_raw(hermes__runtime_evaluate_javascript(
                &*self,
                // FIXME: (@hahnlee)
                &*buffer as *const _ as *const libc::c_void,
                source_url.as_ptr(),
            ))
            .unwrap()
        }
    }
}

#[derive(Debug)]
pub struct OwnedHermesRuntime {
    runtime: std::ptr::NonNull<HermesRuntime>,
}

impl OwnedHermesRuntime {
    pub fn new() -> OwnedHermesRuntime {
        let ptr = unsafe { hermes__make_hermes_runtime() };
        OwnedHermesRuntime::from_raw(ptr)
    }

    pub fn from_raw(ptr: *mut HermesRuntime) -> OwnedHermesRuntime {
        OwnedHermesRuntime {
            runtime: std::ptr::NonNull::new(ptr).unwrap(),
        }
    }
}

impl Deref for OwnedHermesRuntime {
    type Target = HermesRuntime;
    fn deref(&self) -> &Self::Target {
        unsafe { self.runtime.as_ref() }
    }
}

impl Drop for OwnedHermesRuntime {
    fn drop(&mut self) {
        unsafe { hermes__runtime_delete(self.runtime.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::compile_js;
    use crate::jsi::buffer::StringBuffer;
    use crate::jsi::runtime::Runtime;
    use crate::runtime::{HermesRuntime, OwnedHermesRuntime};
    use std::ops::Deref;

    #[test]
    fn check_version() {
        assert_eq!(HermesRuntime::get_bytecode_version(), 89);
    }

    #[test]
    fn check_create() {
        let runtime = OwnedHermesRuntime::new();
        assert_eq!(runtime.is_inspectable(), true);
    }

    #[test]
    fn check_bytecode() {
        let valid = compile_js("x + 2", false).unwrap();
        assert_eq!(HermesRuntime::is_hermes_bytecode(&valid), true)
    }

    #[test]
    fn check_bytecode_err() {
        let invalid: [u8; 0] = [];
        assert_eq!(HermesRuntime::is_hermes_bytecode(&invalid), false);
    }

    #[test]
    fn check_evaluate_javascript() {
        let runtime = OwnedHermesRuntime::new();
        let value = runtime.evaluate_javascript(StringBuffer::new("1 + 1").deref(), "");
        assert_eq!(value.is_number(runtime.deref()), true);
        assert_eq!(value.as_number(), 2.0);
    }

    #[test]
    fn check_global() {
        let runtime = OwnedHermesRuntime::new();
        runtime.evaluate_javascript(StringBuffer::new("x = 321").deref(), "");
        let global = runtime.global();
        let value = global.get_property::<HermesRuntime>(&runtime, "x");
        assert_eq!(value.is_number(runtime.deref()), true);
        assert_eq!(value.as_number(), 321.0);
    }
}
