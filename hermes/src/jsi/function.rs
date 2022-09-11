use crate::jsi::runtime::Runtime;
use crate::support::Opaque;

extern "C" {
    fn jsi__function_createFromHostFunction(runtime: *const libc::c_void) -> Function;
}

#[repr(C)]
#[derive(Debug)]
pub struct Function(Opaque);

impl Function {
    pub fn from_host_function<T: Runtime>(runtime: &T) -> Function {
        unsafe {
            jsi__function_createFromHostFunction(&*runtime as *const _ as *const libc::c_void)
        }
    }
}
