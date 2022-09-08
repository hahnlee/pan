use crate::jsi::runtime::Runtime;
use crate::support::Opaque;

extern "C" {
    fn jsi__value_isUndefined(value: *const Value) -> bool;
    fn jsi__value_isNumber(value: *const Value, runtime: *const libc::c_void) -> bool;
    fn jsi__value_delete(value: *const Value);
}

#[repr(C)]
#[derive(Debug)]
pub struct Value(Opaque);

impl Value {
    pub fn is_undefined(&self) -> bool {
        unsafe { jsi__value_isUndefined(&*self) }
    }

    pub fn is_number<T: Runtime>(&self, runtime: *const T) -> bool {
        unsafe { jsi__value_isNumber(&*self, runtime as *const _ as *const libc::c_void) }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { jsi__value_delete(&*self) }
    }
}
