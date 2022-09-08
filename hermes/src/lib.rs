pub mod handle;
pub mod jsi;
pub mod runtime;
pub mod support;

use std::ffi::CString;

extern "C" {
    fn hermes__compileJS(
        code: *const i8,
        data: &*mut u8,
        size: *mut libc::size_t,
        optimize: bool,
    ) -> bool;
}

pub fn compile_js(code: &str, optimize: bool) -> Result<&[u8], ()> {
    let bytecode = std::ptr::null_mut::<u8>();
    let mut size: usize = 0;

    let code = CString::new(code).unwrap();

    let result = unsafe { hermes__compileJS(code.as_ptr(), &bytecode, &mut size, optimize) };
    if !result {
        return Err(());
    }

    let slice = unsafe { std::slice::from_raw_parts(bytecode, size) };

    return Ok(slice);
}

#[cfg(test)]
mod tests {
    use crate::compile_js;
    use crate::jsi::buffer::{Buffer, StringBuffer};
    use crate::jsi::runtime::Runtime;
    use crate::runtime;
    use std::ops::Deref;

    #[test]
    fn check_version() {
        assert_eq!(runtime::HermesRuntime::get_bytecode_version(), 89);
    }

    #[test]
    fn check_runtime() {
        let runtime = runtime::HermesRuntime::new();
        assert_eq!(runtime.is_inspectable(), true);
    }

    #[test]
    fn check_compile_js() {
        let valid = compile_js("x = 1", false);
        assert_eq!(valid.is_ok(), true);
    }

    #[test]
    fn check_compile_js_err() {
        let invalid = compile_js("fn test() {}", false);
        assert_eq!(invalid.is_err(), true);
    }

    #[test]
    fn check_bytecode() {
        let valid = compile_js("x + 2", false).unwrap();
        assert_eq!(runtime::HermesRuntime::is_hermes_bytecode(&valid), true)
    }

    #[test]
    fn check_bytecode_err() {
        let invalid: [u8; 0] = [];
        assert_eq!(runtime::HermesRuntime::is_hermes_bytecode(&invalid), false);
    }

    #[test]
    fn create_string_buffer() {
        let buffer = StringBuffer::new("Hello World!");
        assert_eq!(buffer.size(), 12);
    }

    #[test]
    fn check_evaluate_javascript() {
        let runtime = runtime::HermesRuntime::new();
        let value = runtime.evaluate_javascript(StringBuffer::new("1 + 1").deref(), "");
        assert_eq!(value.is_number(runtime.deref()), true);
    }
}
