use crate::handle::Local;
use crate::support::Opaque;

extern "C" {
    fn hermes__makeHermesRuntime() -> *mut HermesRuntime;
    fn hermes__getBytecodeVersion() -> u32;
    fn hermes__runtime_isInspectable(this: *const HermesRuntime) -> u8;
}

#[repr(C)]
#[derive(Debug)]
pub struct HermesRuntime(Opaque);

impl HermesRuntime {
    pub fn get_bytecode_version() -> u32 {
        unsafe { hermes__getBytecodeVersion() }
    }

    pub fn new<'s>() -> Local<'s, HermesRuntime> {
        unsafe { Local::from_raw(hermes__makeHermesRuntime()).unwrap() }
    }

    pub fn is_inspectable(&self) -> bool {
        let out = unsafe { hermes__runtime_isInspectable(&*self) };

        return out == 1;
    }
}
