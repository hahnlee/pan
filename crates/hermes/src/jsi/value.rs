use crate::cpp::string::CppString;
use crate::handle::Local;
use crate::jsi::object::Object;
use crate::jsi::runtime::Runtime;
use crate::support::Opaque;

extern "C" {
    fn jsi__value_from_string(
        runtime: *const libc::c_void,
        ptr: *const u8,
        size: usize,
    ) -> *const Value;
    fn jsi__value_from_number(value: f64) -> *const Value;
    fn jsi__value_is_undefined(value: *const Value) -> bool;
    fn jsi__value_is_number(value: *const Value) -> bool;
    fn jsi__value_is_string(value: *const Value) -> bool;
    fn jsi__value_delete(value: *const Value);
    fn jsi__value_as_number(value: *const Value) -> f64;
    fn jsi__value_as_object(value: *const Value, runtime: *const libc::c_void) -> *const Object;
    fn jsi__offset_from_ptr(ptr: *const Value, offset: usize) -> *const Value;
    fn jsi__value_to_bytes(
        prt: *const Value,
        runtime: *const libc::c_void,
        size: *mut usize,
    ) -> *const libc::c_void;
}

#[repr(C)]
#[derive(Debug)]
pub struct Value(Opaque);

impl Value {
    pub fn from_str<'s, T: Runtime>(string: &str, runtime: &T) -> Local<'s, Value> {
        unsafe {
            Value::from_raw(jsi__value_from_string(
                &*runtime as *const _ as *const libc::c_void,
                string.as_ptr(),
                string.len(),
            ))
        }
    }

    pub fn from_number<'s>(number: f64) -> Local<'s, Value> {
        unsafe { Value::from_raw(jsi__value_from_number(number)) }
    }

    pub fn from_raw<'s>(ptr: *const Value) -> Local<'s, Value> {
        unsafe { Local::from_raw(ptr).unwrap() }
    }

    pub fn is_undefined(&self) -> bool {
        unsafe { jsi__value_is_undefined(&*self) }
    }

    pub fn is_number(&self) -> bool {
        unsafe { jsi__value_is_number(&*self) }
    }

    pub fn is_string(&self) -> bool {
        unsafe { jsi__value_is_string(&*self) }
    }

    pub fn as_number(&self) -> f64 {
        unsafe { jsi__value_as_number(&*self) }
    }

    pub fn as_object<'s, T: Runtime>(&self, runtime: &T) -> Local<'s, Object> {
        println!("changed");
        unsafe {
            Object::from_raw(jsi__value_as_object(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
            ))
        }
    }

    pub fn to_string<T: Runtime>(&self, runtime: &T) -> String {
        let mut size: usize = 0;
        let ptr = unsafe {
            jsi__value_to_bytes(
                &*self,
                &*runtime as *const _ as *const libc::c_void,
                &mut size,
            )
        };

        let str = CppString::from_raw(ptr);
        str.to_string()
    }

    // TODO?: (@hahnlee) this function looks like unnecessary
    pub fn offset<'s>(&self, offset: usize) -> Local<'s, Value> {
        unsafe { Value::from_raw(jsi__offset_from_ptr(&*self, offset)) }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { jsi__value_delete(&*self) }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::{
        jsi::{buffer::StringBuffer, runtime::Runtime},
        runtime::OwnedHermesRuntime,
    };

    #[test]
    fn check_to_str() {
        let runtime = OwnedHermesRuntime::new();
        let value = runtime.evaluate_javascript(StringBuffer::new("'buffer\0test'").deref(), "");
        let out = value.to_string(runtime.deref());
        assert_eq!(out, "buffer\0test");
    }
}
