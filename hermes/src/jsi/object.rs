use crate::handle::Local;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

use std::ffi::CString;

extern "C" {
    fn jsi__object_delete(object: *const Object);
    fn jsi__object_getProperty(
        Object: *const Object,
        runtime: *const libc::c_void,
        name: *const libc::c_char,
    ) -> *const Value;
    fn jsi__object_setProperty(
        Object: *const Object,
        runtime: *const libc::c_void,
        name: *const libc::c_char,
        value: *const libc::c_void,
    );
}

#[repr(C)]
#[derive(Debug)]
pub struct Object(Opaque);

impl Object {
    pub fn from_ptr<'s>(ptr: *const Object) -> Local<'s, Object> {
        unsafe { Local::from_raw(ptr).unwrap() }
    }

    pub fn get_property<T: Runtime>(&self, runtime: *const T, name: &str) -> Local<'_, Value> {
        let name = CString::new(name).unwrap();

        unsafe {
            Value::from_ptr(jsi__object_getProperty(
                &*self,
                runtime as *const _ as *const libc::c_void,
                name.as_ptr(),
            ))
        }
    }

    pub fn set_property<T: Runtime>(&self, runtime: &T, name: &str, value: *const libc::c_void) {
        let name = CString::new(name).unwrap();

        unsafe {
            jsi__object_setProperty(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
                name.as_ptr(),
                value,
            );
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { jsi__object_delete(&*self) }
    }
}
