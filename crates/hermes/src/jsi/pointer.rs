use crate::cpp::string::CppString;
use crate::jsi::runtime::Runtime;
use crate::support::Opaque;

use std::ffi::CString;

extern "C" {
    fn jsi__prop_name_id_for_utf8(
        runtime: *const libc::c_void,
        name: *const libc::c_char,
    ) -> *mut InternalPropNameID;
    fn jsi__prop_name_id_utf8(
        prop: *const InternalPropNameID,
        runtime: *const libc::c_void,
    ) -> *const libc::c_void;
}

#[repr(C)]
#[derive(Debug)]
struct InternalPropNameID(Opaque);

pub struct PropNameID(*mut InternalPropNameID);

impl PropNameID {
    pub fn from_str<T: Runtime>(runtime: &T, name: &str) -> PropNameID {
        let name = CString::new(name).unwrap();
        unsafe {
            PropNameID(jsi__prop_name_id_for_utf8(
                &*runtime as *const _ as *const libc::c_void,
                name.as_ptr(),
            ))
        }
    }

    pub fn to_string<'s, T: Runtime>(&mut self, runtime: &T) -> String {
        let mut cpp_str = unsafe {
            CppString::from_raw(jsi__prop_name_id_utf8(
                self.0,
                &*runtime as *const _ as *const libc::c_void,
            ))
        };

        return String::from(cpp_str.to_str());
    }

    pub fn to_ptr(&self) -> *const libc::c_void {
        self.0 as *const libc::c_void
    }
}
