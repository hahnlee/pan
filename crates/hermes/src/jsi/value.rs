use crate::handle::Local;
use crate::support::Opaque;

extern "C" {
    fn jsi__value_new_number(value: f64) -> *const Value;
    fn jsi__value_is_undefined(value: *const Value) -> bool;
    fn jsi__value_is_number(value: *const Value) -> bool;
    fn jsi__value_is_string(value: *const Value) -> bool;
    fn jsi__value_delete(value: *const Value);
    fn jsi__value_as_number(value: *const Value) -> f64;
    fn jsi__offset_from_ptr(ptr: *const Value, offset: usize) -> *const Value;
}

#[repr(C)]
#[derive(Debug)]
pub struct Value(Opaque);

impl Value {
    pub fn from_number<'s>(number: f64) -> Local<'s, Value> {
        unsafe { Value::from_raw(jsi__value_new_number(number)) }
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
