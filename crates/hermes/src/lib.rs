pub mod buffer;
pub mod handle;
pub mod jsi;
pub mod runtime;

mod cpp;
mod support;

extern "C" {
    fn hermes__compile_js(
        code: *const u8,
        code_size: usize,
        data: &*mut u8,
        size: *mut usize,
        optimize: bool,
    ) -> bool;
}

pub fn compile_js(code: &str, optimize: bool) -> Result<&[u8], ()> {
    let bytecode = std::ptr::null_mut::<u8>();
    let mut size: usize = 0;

    let len = if code.ends_with("\0") { code.len() - 1 } else { code.len() };

    let result = unsafe { hermes__compile_js(code.as_ptr(), len, &bytecode, &mut size, optimize) };
    if !result {
        return Err(());
    }

    let slice = unsafe { std::slice::from_raw_parts(bytecode, size) };

    return Ok(slice);
}

#[cfg(test)]
mod tests {
    use crate::compile_js;

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
}
