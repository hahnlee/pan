use std::fs;

use hermes::buffer::MemoryBuffer;
use hermes::jsi::runtime::Runtime;
use hermes::runtime::HermesRuntime;
use hermes::runtime::OwnedHermesRuntime;

pub mod version;

// TODO: (@hahnlee) provide FFI function
pub fn initialize() -> OwnedHermesRuntime {
    OwnedHermesRuntime::new()
}

// TODO: (@hahnlee) provide error code for FFI
pub fn run_js(runtime: &HermesRuntime, file_path: &str) {
    let file = fs::read(file_path).unwrap();
    let buffer = MemoryBuffer::from_bytes(&file);

    // FIXME: (@hahnlee) convert file_path to url
    runtime.evaluate_javascript(&buffer, file_path);
}
