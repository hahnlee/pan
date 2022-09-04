extern "C" {
    fn hermes__makeHermesRuntime() -> *mut libc::c_void;
    fn hermes__getBytecodeVersion() -> u32;
    fn hermes__runtime_isInspectable(this: *mut libc::c_void) -> u8;
}

#[repr(C)]
#[derive(Debug)]
pub struct HermesRuntime {
    ptr: *mut libc::c_void,
}

impl HermesRuntime {
    pub fn get_bytecode_version() -> u32 {
        unsafe { hermes__getBytecodeVersion() }
    }

    pub fn new() -> Self {
        let ptr = unsafe { hermes__makeHermesRuntime() };

        HermesRuntime { ptr }
    }

    pub fn is_inspectable(&self) -> bool {
        let out = unsafe { hermes__runtime_isInspectable(self.ptr) };

        return out == 1;
    }
}

impl Drop for HermesRuntime {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.ptr);
        }
    }
}
