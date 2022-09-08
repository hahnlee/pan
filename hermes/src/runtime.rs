use crate::handle::Local;
use crate::jsi::buffer::Buffer;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

use std::ffi::CString;

extern "C" {
    fn hermes__makeHermesRuntime() -> *mut HermesRuntime;
    fn hermes__runtime_isHermesBytecode(data: *const u8, len: libc::size_t) -> bool;
    fn hermes__runtime_isInspectable(runtime: *const HermesRuntime) -> bool;
    fn hermes__runtime_getBytecodeVersion() -> u32;
    fn hermes__runtime_evaluateJavaScript(
        runtime: *const HermesRuntime,
        buffer: *const libc::c_void,
        source_url: *const libc::c_char,
    ) -> *const Value;
    fn hermes__runtime_delete(runtime: *const HermesRuntime);
}

#[repr(C)]
#[derive(Debug)]
pub struct HermesRuntime(Opaque);

impl HermesRuntime {
    pub fn is_hermes_bytecode(data: &[u8]) -> bool {
        unsafe { hermes__runtime_isHermesBytecode(data.as_ptr(), data.len()) }
    }

    pub fn get_bytecode_version() -> u32 {
        unsafe { hermes__runtime_getBytecodeVersion() }
    }

    pub fn new<'s>() -> Local<'s, HermesRuntime> {
        unsafe { Local::from_raw(hermes__makeHermesRuntime()).unwrap() }
    }

    pub fn is_inspectable(&self) -> bool {
        unsafe { hermes__runtime_isInspectable(&*self) }
    }
}

impl Runtime for HermesRuntime {
    fn evaluate_javascript<T: Buffer>(&self, buffer: &T, source_url: &str) -> Local<'_, Value> {
        unsafe {
            Local::from_raw(hermes__runtime_evaluateJavaScript(
                &*self,
                // FIXME: (@hahnlee)
                &*buffer as *const _ as *const libc::c_void,
                CString::new(source_url).unwrap().as_ptr(),
            ))
            .unwrap()
        }
    }
}

impl Drop for HermesRuntime {
    fn drop(&mut self) {
        unsafe { hermes__runtime_delete(&*self) }
    }
}
