use crate::handle::Local;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

use std::ffi::CString;

use super::function::{Function, InternalFunction};

extern "C" {
    fn jsi__object_new(runtime: *const libc::c_void) -> *const Object;
    fn jsi__object_get_property(
        Object: *const Object,
        runtime: *const libc::c_void,
        name: *const libc::c_char,
    ) -> *const Value;
    fn jsi__object_set_property(
        Object: *const Object,
        runtime: *const libc::c_void,
        name: *const libc::c_char,
        value: *const Value,
    );
    fn jsi__object_set_function(
        Object: *const Object,
        runtime: *const libc::c_void,
        name: *const libc::c_char,
        value: *const libc::c_void,
    );
    fn jsi__object_as_function(
        Object: *const Object,
        runtime: *const libc::c_void,
    ) -> *const InternalFunction;
    fn jsi__object_to_value(Object: *const Object, runtime: *const libc::c_void) -> *const Value;
    fn jsi__object_delete(object: *const Object);
}

#[repr(C)]
#[derive(Debug)]
pub struct Object(Opaque);

impl Object {
    pub fn new<'s, T: Runtime>(runtime: &T) -> Local<'s, Object> {
        unsafe {
            Object::from_raw(jsi__object_new(
                &*runtime as *const _ as *const libc::c_void,
            ))
        }
    }

    pub fn from_raw<'s>(ptr: *const Object) -> Local<'s, Object> {
        unsafe { Local::from_raw(ptr).unwrap() }
    }

    pub fn get_property<T: Runtime>(&self, runtime: &T, name: &str) -> Local<'_, Value> {
        let name = CString::new(name).unwrap();

        unsafe {
            Value::from_raw(jsi__object_get_property(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
                name.as_ptr(),
            ))
        }
    }

    pub fn set_property<T: Runtime>(&self, runtime: &T, name: &str, value: &Value) {
        let name = CString::new(name).unwrap();

        unsafe {
            jsi__object_set_property(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
                name.as_ptr(),
                &*value,
            );
        }
    }

    pub fn set_function<T: Runtime>(&self, runtime: &T, name: &str, function: Function) {
        let name = CString::new(name).unwrap();

        unsafe {
            jsi__object_set_function(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
                name.as_ptr(),
                function.to_ptr(),
            );
        }
    }

    pub fn as_function<T: Runtime>(&self, runtime: &T) -> Function {
        unsafe {
            Function::from_raw(jsi__object_as_function(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
            ))
        }
    }

    pub fn to_value<'s, T: Runtime>(&self, runtime: &T) -> Local<'s, Value> {
        unsafe {
            Value::from_raw(jsi__object_to_value(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
            ))
        }
    }

    pub fn to_ptr(&self) -> *const libc::c_void {
        &*self as *const _ as *const libc::c_void
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe { jsi__object_delete(&*self) }
    }
}
