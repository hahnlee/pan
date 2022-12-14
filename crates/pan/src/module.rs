use hermes::buffer::MemoryBuffer;
use hermes::handle::Local;
use hermes::jsi::function::Function;
use hermes::jsi::object::Object;
use hermes::jsi::runtime::Runtime;
use hermes::jsi::value::Value;
use hermes::runtime::HermesRuntime;

use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;

use crate::runtime::PanRuntime;

const MODULE_PREFIX: &[u8] = "(function(exports, module, __filename, __dirname) {\n".as_bytes();
const MODULE_SUFFIX: &[u8] = "\n});".as_bytes();

pub fn wrap_module_code(data: &[u8]) -> Vec<u8> {
    [MODULE_PREFIX, data, MODULE_SUFFIX, &[0]].concat()
}

pub fn evaluate_module(
    runtime: &HermesRuntime,
    absolute_path: PathBuf,
    stack: &mut Vec<PathBuf>,
    modules: &mut HashMap<String, Local<Value>>,
) -> *const Value {
    let path = absolute_path.to_str().unwrap().to_string();
    let cached = modules.get(&path);
    if cached.is_some() {
        return cached
            .unwrap()
            .as_object(runtime)
            .get_property(runtime, "exports")
            .deref();
    }

    let file = fs::read(&absolute_path).unwrap();
    let data = match path.ends_with(".hbc") {
        true => [file, vec![0]].concat(),
        false => wrap_module_code(&file),
    };

    let buffer = MemoryBuffer::from_bytes(&data);

    let parent = absolute_path
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let source_url = format!("file://{}", path);

    stack.push(absolute_path);

    let module = runtime.evaluate_javascript(&buffer, &source_url);

    let function = module.as_object(runtime).as_function(runtime);

    let module = Object::new(runtime);
    let exports = Object::new(runtime).to_value(runtime);

    module.set_property(runtime, "exports", exports.deref());
    let module = module.to_value(runtime);

    function.call(
        runtime,
        &[
            exports,
            module,
            Value::from_str(&path, runtime),
            Value::from_str(&parent, runtime),
        ],
    );

    stack.pop();

    module.as_object(runtime).get_property(runtime, "exports");

    modules.insert(path, module);

    return exports.deref();
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

            evaluate_module(&runtime, absolute_path, &mut pan.stack, &mut pan.modules)
        },
    );

    pan.hermes
        .deref()
        .global()
        .set_function(pan.hermes.deref(), "require", require);
}

fn find_module_path(current: &PathBuf, module: &str) -> Result<String, ()> {
    let base_path = to_search_path(current, module);

    if base_path.starts_with("/") {
        let file = load_as_file(base_path.as_str());
        if file.is_ok() {
            return file;
        }

        let dir = load_as_directory(&base_path.as_str());
        if dir.is_ok() {
            return dir;
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

    // NOTE: (@hahnlee) .hbc file has higher property
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

/// LOAD_AS_DIRECTORY
///
/// [reference](https://nodejs.org/api/modules.html#all-together)
fn load_as_directory(path: &str) -> Result<String, ()> {
    let package_json = format!("{}/package.json", path);
    if PathBuf::from(&package_json).is_file() {
        // TODO: (@hahnlee) impl package.json
        return Err(());
    }

    return load_index(path);
}

/// LOAD_INDEX
///
/// [reference](https://nodejs.org/api/modules.html#all-together)
fn load_index(path: &str) -> Result<String, ()> {
    // NOTE: (@hahnlee) .hbc file has higher property
    let hbc = format!("{}/index.hbc", path);
    if PathBuf::from(&hbc).is_file() {
        return Ok(hbc);
    }

    let js = format!("{}/index.js", path);
    if PathBuf::from(&js).is_file() {
        return Ok(js);
    }

    let json = format!("{}/index.json", path);
    if PathBuf::from(&json).is_file() {
        return Ok(json);
    }

    let node = format!("{}/index.node", path);
    if PathBuf::from(&node).is_file() {
        return Ok(node);
    }

    Err(())
}
