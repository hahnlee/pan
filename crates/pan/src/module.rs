use hermes::buffer::MemoryBuffer;
use hermes::jsi::function::Function;
use hermes::jsi::runtime::Runtime;
use hermes::jsi::value::Value;
use hermes::runtime::HermesRuntime;

use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

use crate::runtime::PanRuntime;

const MODULE_PREFIX: &[u8] =
    "(function(exports, require, module, __filename, __dirname) {".as_bytes();
const MODULE_SUFFIX: &[u8] = "});".as_bytes();

pub fn evaluate_module(
    runtime: &HermesRuntime,
    absolute_path: PathBuf,
    stack: &mut Vec<PathBuf>,
) -> *const Value {
    let file = fs::read(&absolute_path).unwrap();

    let data = [MODULE_PREFIX, &file, MODULE_SUFFIX, &[0]].concat();
    let buffer = MemoryBuffer::from_bytes(&data);
    let source_url = format!("file://{}", absolute_path.to_str().unwrap());

    stack.push(absolute_path);

    let value = runtime.evaluate_javascript(&buffer, &source_url);
    // TODO: (@hahnlee) call with argument

    stack.pop();

    // TODO: (@hahnlee) change function return
    return value.deref();
}

pub fn bind_require(pan: &mut PanRuntime) {
    let require = Function::from_host_function(
        pan.hermes.deref(),
        "require",
        1,
        |runtime_ptr, _, args, _| {
            let runtime = HermesRuntime::from_raw(runtime_ptr);
            let name = args[0].to_string::<HermesRuntime>(&runtime);

            let current_path = pan.stack.last().unwrap();

            let module_path = find_module_path(&current_path, &name).unwrap();
            let absolute_path = PathBuf::from(module_path).canonicalize().unwrap();

            evaluate_module(&runtime, absolute_path, &mut pan.stack)
        },
    );

    pan.hermes
        .deref()
        .global()
        .set_property(pan.hermes.deref(), "require", require.to_ptr());
}

fn find_module_path(current: &PathBuf, module: &str) -> Result<String, ()> {
    let base_path = to_search_path(current, module);

    if base_path.starts_with("/") {
        let file = load_as_file(base_path.as_str());
        if file.is_ok() {
            return file;
        }
    }

    Err(())
}

fn to_search_path(current: &PathBuf, module: &str) -> String {
    if module.starts_with("./") || module.starts_with("../") {
        let mut parent = current.parent().unwrap().to_path_buf();
        parent.push(module);

        let path = parent.to_str().unwrap();
        return path.to_string();
    };

    module.to_string()
}

/// LOAD_AS_FILE
///
/// [reference](https://nodejs.org/api/modules.html#all-together)
fn load_as_file(path: &str) -> Result<String, ()> {
    if PathBuf::from(path).is_file() {
        return Ok(path.to_string());
    }

    // NOTE: (@hahnlee)
    let hbc = format!("{}.hbc", path);
    if PathBuf::from(&hbc).is_file() {
        return Ok(hbc);
    }

    let js = format!("{}.js", path);
    if PathBuf::from(&js).is_file() {
        return Ok(js);
    }

    let json = format!("{}.json", path);
    if PathBuf::from(&json).is_file() {
        return Ok(json);
    }

    let node = format!("{}.node", path);
    if PathBuf::from(&node).is_file() {
        return Ok(node);
    }

    Err(())
}
