pub mod runtime;
pub mod version;

mod module;

use hermes::compile_js;
use runtime::PanRuntime;

use crate::module::wrap_module_code;

// TODO: (@hahnlee) provide FFI function
pub fn initialize() -> PanRuntime {
    let mut runtime = PanRuntime::new();
    runtime.initialize();
    return runtime;
}

// TODO: (@hahnlee) provide error code for FFI
pub fn run_js(runtime: &mut PanRuntime, file_path: &str) {
    runtime.run(file_path);
}

pub fn compile(input: &[u8], optimize: bool) -> Vec<u8> {
    let wrapped = wrap_module_code(input);
    let code = String::from_utf8(wrapped).unwrap();

    let result = compile_js(&code, optimize);
    return result.unwrap().to_vec();
}
