use crate::handle::Local;
use crate::support::Opaque;

extern "C" {
    fn hermes__isHermesBytecode(data: *const u8, len: libc::size_t) -> u8;
    fn hermes__runtime_isInspectable(this: *const HermesRuntime) -> u8;
    fn hermes__makeHermesRuntime() -> *mut HermesRuntime;
    fn hermes__getBytecodeVersion() -> u32;
}

#[repr(C)]
#[derive(Debug)]
pub struct HermesRuntime(Opaque);

impl HermesRuntime {
    pub fn is_hermes_bytecode(data: &[u8]) -> bool {
        unsafe { hermes__isHermesBytecode(data.as_ptr(), data.len()) == 1 }
    }

    pub fn get_bytecode_version() -> u32 {
        unsafe { hermes__getBytecodeVersion() }
    }

    pub fn new<'s>() -> Local<'s, HermesRuntime> {
        unsafe { Local::from_raw(hermes__makeHermesRuntime()).unwrap() }
    }

    pub fn is_inspectable(&self) -> bool {
        unsafe { hermes__runtime_isInspectable(&*self) == 1 }
    }
}
