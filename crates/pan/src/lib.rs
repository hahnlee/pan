use runtime::PanRuntime;

pub mod runtime;
pub mod version;

mod module;

// TODO: (@hahnlee) provide FFI function
pub fn initialize() -> PanRuntime {
    PanRuntime::new()
}

// TODO: (@hahnlee) provide error code for FFI
pub fn run_js(runtime: &mut PanRuntime, file_path: &str) {
   runtime.run(file_path);
}
