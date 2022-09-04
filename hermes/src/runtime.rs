use crate::jsi::runtime::Runtime;

use crate::support::Opaque;

use std::ops::Deref;

extern "C" {
    fn hermes__makeHermesRuntime() -> *mut HermesRuntime;
    fn hermes__getBytecodeVersion() -> u32;
}

pub fn get_bytecode_version() -> u32 {
    unsafe { hermes__getBytecodeVersion() }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HermesRuntime(Opaque);

impl Deref for HermesRuntime {
    type Target = Runtime;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const Self::Target) }
    }
}

pub fn make_hermes_runtime() -> *const HermesRuntime {
    unsafe { hermes__makeHermesRuntime() }
}
